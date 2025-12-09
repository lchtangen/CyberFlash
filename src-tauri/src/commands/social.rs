use tauri::{command, AppHandle, Manager};
use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::general_purpose};
use std::fs;
use std::path::PathBuf;
use rss::Channel;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FeedItem {
    pub title: String,
    pub link: String,
    pub description: String,
    pub pub_date: String,
    pub source: String,
}

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
pub async fn fetch_rss_feed(url: String, source: String) -> Result<Vec<FeedItem>, String> {
    let content = reqwest::get(&url)
        .await
        .map_err(|e| e.to_string())?
        .bytes()
        .await
        .map_err(|e| e.to_string())?;

    let channel = Channel::read_from(&content[..]).map_err(|e| e.to_string())?;
    
    let items = channel.items().iter().take(10).map(|item| {
        FeedItem {
            title: item.title().unwrap_or("No Title").to_string(),
            link: item.link().unwrap_or("").to_string(),
            description: item.description().unwrap_or("").to_string(),
            pub_date: item.pub_date().unwrap_or("").to_string(),
            source: source.clone(),
        }
    }).collect();

    Ok(items)
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
pub async fn share_config(name: String, _content: String) -> Result<String, String> {
    // Mock implementation for now
    Ok(format!("Config '{}' shared successfully", name))
}

#[command]
pub async fn download_feed_item(app: AppHandle, url: String, title: String) -> Result<String, String> {
    let download_dir = app.path().download_dir().map_err(|e| e.to_string())?;
    // Sanitize filename
    let safe_title = title.replace(|c: char| !c.is_alphanumeric(), "_");
    let filename = format!("{}.zip", safe_title); // Assume zip for now
    let file_path = download_dir.join(filename);

    let response = reqwest::get(&url).await.map_err(|e| e.to_string())?;
    let content = response.bytes().await.map_err(|e| e.to_string())?;
    
    fs::write(&file_path, content).map_err(|e| e.to_string())?;
    
    Ok(file_path.to_string_lossy().to_string())
}

#[command]
pub async fn search_community(app: AppHandle, query: String) -> Result<Vec<FeedItem>, String> {
    let mut results = Vec::new();
    let query_lower = query.to_lowercase();

    // 1. Search Community Repos
    if let Ok(repos) = fetch_community_repos(app.clone()).await {
        for repo in repos {
            if repo.name.to_lowercase().contains(&query_lower) || repo.description.to_lowercase().contains(&query_lower) {
                results.push(FeedItem {
                    title: repo.name,
                    link: repo.download_url,
                    description: repo.description,
                    pub_date: "".to_string(),
                    source: "Community Repo".to_string(),
                });
            }
        }
    }

    // 2. Search RSS Feeds (Mock search across known feeds)
    // In a real app, we'd cache feeds and search the cache.
    // For now, let's just return what we have.
    
    Ok(results)
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
