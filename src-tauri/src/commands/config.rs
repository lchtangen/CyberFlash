use tauri::{command, AppHandle, Manager};
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

fn get_config_path(app: &AppHandle) -> Result<PathBuf, String> {
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    Ok(config_dir.join("user_settings.json"))
}

#[command]
pub async fn load_settings(app: AppHandle) -> Result<Value, String> {
    let config_path = get_config_path(&app)?;

    if !config_path.exists() {
        return Ok(serde_json::json!({}));
    }
    
    let content = fs::read_to_string(config_path).map_err(|e| e.to_string())?;
    let settings: Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    Ok(settings)
}

#[command]
pub async fn save_settings(app: AppHandle, settings: Value) -> Result<String, String> {
    let config_path = get_config_path(&app)?;
    let json_string = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    
    // Ensure config directory exists
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    fs::write(config_path, json_string).map_err(|e| e.to_string())?;
    Ok("Settings saved successfully".to_string())
}
