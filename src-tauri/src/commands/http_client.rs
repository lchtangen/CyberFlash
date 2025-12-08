use tauri::{command, AppHandle, Emitter};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::commands::config;

pub async fn download_file_with_progress<F>(url: String, save_path: String, on_progress: F) -> Result<(), String> 
where F: Fn(f64) + Send + Sync + 'static 
{
    let client = reqwest::Client::new();
    let mut response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    
    let total_size = response.content_length().unwrap_or(0);
    let mut file = File::create(&save_path).map_err(|e| e.to_string())?;
    let mut downloaded: u64 = 0;

    while let Some(chunk) = response.chunk().await.map_err(|e| e.to_string())? {
        file.write_all(&chunk).map_err(|e| e.to_string())?;
        downloaded += chunk.len() as u64;
        
        if total_size > 0 {
            let progress = (downloaded as f64 / total_size as f64) * 100.0;
            on_progress(progress);
        }
    }
    Ok(())
}

#[command]
pub async fn download_file(app: AppHandle, url: String, save_path: String) -> Result<String, String> {
    let app_clone = app.clone();
    download_file_with_progress(url, save_path, move |progress| {
        let _ = app_clone.emit("download-progress", progress);
    }).await?;

    Ok("Download complete".to_string())
}

#[command]
pub async fn download_file_auto(app: AppHandle, url: String, filename: String) -> Result<String, String> {
    let settings = config::load_settings(app.clone()).await?;
    let default_path = "./downloads";
    let save_dir = settings["save_path"].as_str().unwrap_or(default_path);
    let save_path = Path::new(save_dir).join(filename);

    if !Path::new(save_dir).exists() {
        std::fs::create_dir_all(save_dir).map_err(|e| e.to_string())?;
    }

    let app_clone = app.clone();
    download_file_with_progress(url, save_path.to_string_lossy().to_string(), move |progress| {
        let _ = app_clone.emit("download-progress", progress);
    }).await?;

    Ok("Download complete".to_string())
}
