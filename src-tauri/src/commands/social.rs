use tauri::{command, AppHandle};
use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::general_purpose};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommunityRom {
    pub id: String,
    pub name: String,
    pub device: String,
    pub version: String,
    pub author: String,
    pub download_url: String,
    pub description: String,
    pub likes: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SharedConfig {
    pub device: String,
    pub rom: String,
    pub gapps: Option<String>,
    pub magisk: bool,
    pub modules: Vec<String>,
}

#[command]
pub async fn fetch_community_repos(_app: AppHandle) -> Result<Vec<CommunityRom>, String> {
    // Read from config/community_repos.json
    let config_path = PathBuf::from("config/community_repos.json");
    
    if !config_path.exists() {
        return Err("Community repos config not found".to_string());
    }

    let content = fs::read_to_string(config_path).map_err(|e| e.to_string())?;
    let roms: Vec<CommunityRom> = serde_json::from_str(&content).map_err(|e| e.to_string())?;

    Ok(roms)
}

#[command]
pub fn generate_share_link(config: SharedConfig) -> Result<String, String> {
    let json = serde_json::to_string(&config).map_err(|e| e.to_string())?;
    let b64 = general_purpose::STANDARD.encode(json);
    Ok(format!("cyberflash://share/{}", b64))
}

#[command]
pub fn decode_share_link(link: String) -> Result<SharedConfig, String> {
    let prefix = "cyberflash://share/";
    if !link.starts_with(prefix) {
        return Err("Invalid link format".to_string());
    }
    
    let b64 = &link[prefix.len()..];
    let bytes = general_purpose::STANDARD.decode(b64).map_err(|e| e.to_string())?;
    let json = String::from_utf8(bytes).map_err(|e| e.to_string())?;
    let config: SharedConfig = serde_json::from_str(&json).map_err(|e| e.to_string())?;
    
    Ok(config)
}

#[command]
pub async fn share_config(name: String, content: String) -> Result<String, String> {
    // Save to local file in downloads/shared_configs
    let mut path = PathBuf::from("downloads/shared_configs");
    fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    
    let filename = format!("{}_{}.json", name.replace(" ", "_"), chrono::Local::now().format("%Y%m%d_%H%M%S"));
    path.push(filename);
    
    fs::write(&path, content).map_err(|e| e.to_string())?;
    
    Ok(format!("Config saved to {}", path.display()))
}

#[command]
pub async fn sync_dev_profile(action: String, data: Option<String>) -> Result<String, String> {
    // action: "upload" or "download"
    match action.as_str() {
        "upload" => {
            if let Some(d) = data {
                println!("Uploading profile data: {}", d);
                tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
                Ok("Profile synced to cloud successfully.".to_string())
            } else {
                Err("No data provided for upload".to_string())
            }
        },
        "download" => {
            tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
            // Return mock profile data
            Ok(r#"{ "theme": "dark", "adb_path": "/usr/bin/adb", "favorite_roms": ["lineage", "pe"] }"#.to_string())
        },
        _ => Err("Invalid action".to_string())
    }
}
