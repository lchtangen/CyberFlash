use tauri::command;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use crate::commands::adb;

#[derive(Debug, Serialize, Deserialize)]
pub struct DebloatList {
    pub id: String,
    pub name: String,
    pub description: String,
    pub packages: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DebloatConfig {
    pub lists: Vec<DebloatList>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchOpResult {
    pub item: String,
    pub success: bool,
    pub message: String,
}

#[command]
pub fn get_debloat_lists() -> Result<Vec<DebloatList>, String> {
    let config_path = Path::new("../config/debloat_lists.json");
    // In production, this path might need to be resolved relative to the resource directory
    // For dev, we try relative path. If it fails, we return empty or error.
    
    // Try to read from the workspace root if running from src-tauri
    let content = fs::read_to_string(config_path)
        .or_else(|_| fs::read_to_string("config/debloat_lists.json")) // If running from root?
        .or_else(|_| fs::read_to_string("../../config/debloat_lists.json")) // If running from src-tauri/target/debug
        .map_err(|e| format!("Failed to load debloat lists: {}", e))?;

    let config: DebloatConfig = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse debloat lists: {}", e))?;

    Ok(config.lists)
}

#[command]
pub async fn batch_uninstall(app: tauri::AppHandle, packages: Vec<String>) -> Result<Vec<BatchOpResult>, String> {
    let mut results = Vec::new();
    
    for pkg in packages {
        // We reuse the existing adb::uninstall_package logic but we need to call it directly or via shell
        // adb::uninstall_package returns Result<String, String>
        // Let's just call adb shell pm uninstall directly here to avoid dependency complexity if adb module isn't public enough
        // Or better, use the adb module if it's public.
        
        // Assuming adb::run_adb_shell is available and public
        let cmd = format!("pm uninstall --user 0 {}", pkg);
        match adb::run_adb_shell(app.clone(), cmd).await {
            Ok(output) => {
                if output.contains("Success") {
                    results.push(BatchOpResult { item: pkg, success: true, message: "Uninstalled".into() });
                } else {
                    results.push(BatchOpResult { item: pkg, success: false, message: output });
                }
            },
            Err(e) => {
                results.push(BatchOpResult { item: pkg, success: false, message: e });
            }
        }
    }
    
    Ok(results)
}

#[command]
pub async fn batch_install_apks(app: tauri::AppHandle, file_paths: Vec<String>) -> Result<Vec<BatchOpResult>, String> {
    let mut results = Vec::new();
    
    for path in file_paths {
        let file_name = Path::new(&path).file_name().unwrap_or_default().to_string_lossy().to_string();
        
        match adb::install_apk(app.clone(), path.clone()).await {
            Ok(_) => {
                results.push(BatchOpResult { item: file_name, success: true, message: "Installed".into() });
            },
            Err(e) => {
                results.push(BatchOpResult { item: file_name, success: false, message: e });
            }
        }
    }
    
    Ok(results)
}
