use tauri::command;

#[command]
pub async fn get_connected_devices() -> Result<Vec<String>, String> {
    // Placeholder for ADB devices command
    Ok(vec!["emulator-5554".to_string()])
}
