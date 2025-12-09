use tauri::command;
use serde::{Deserialize, Serialize};

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

#[command]
pub async fn fetch_community_repos() -> Result<Vec<CommunityRom>, String> {
    // In a real app, this would fetch from a remote API.
    // For now, we return a mock list.
    let mock_roms = vec![
        CommunityRom {
            id: "rom_1".to_string(),
            name: "PixelExperience Plus".to_string(),
            device: "OnePlus 7 Pro".to_string(),
            version: "13.0".to_string(),
            author: "jhenrique09".to_string(),
            download_url: "https://example.com/pe_plus.zip".to_string(),
            description: "Pixel Experience for OnePlus 7 Pro with extra customization.".to_string(),
            likes: 1240,
        },
        CommunityRom {
            id: "rom_2".to_string(),
            name: "LineageOS 21".to_string(),
            device: "Generic".to_string(),
            version: "21.0".to_string(),
            author: "LineageOS Team".to_string(),
            download_url: "https://example.com/lineage.zip".to_string(),
            description: "Official LineageOS builds.".to_string(),
            likes: 5432,
        },
        CommunityRom {
            id: "rom_3".to_string(),
            name: "Evolution X".to_string(),
            device: "Xiaomi Mi 11".to_string(),
            version: "8.0".to_string(),
            author: "joeyhuab".to_string(),
            download_url: "https://example.com/evox.zip".to_string(),
            description: "Pixel feel with customization.".to_string(),
            likes: 890,
        },
    ];

    // Simulate network delay
    tokio::time::sleep(std::time::Duration::from_millis(800)).await;

    Ok(mock_roms)
}

#[command]
pub async fn share_config(name: String, content: String) -> Result<String, String> {
    // In a real app, this would POST to a server.
    // We'll just simulate a success and return a fake ID.
    
    println!("Sharing config '{}' with content length: {}", name, content.len());
    
    // Simulate network delay
    tokio::time::sleep(std::time::Duration::from_millis(1200)).await;
    
    Ok(format!("cfg_{}", uuid::Uuid::new_v4().to_string().chars().take(8).collect::<String>()))
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
