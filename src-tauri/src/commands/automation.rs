use tauri::command;

#[command]
pub async fn start_flash_process() -> Result<String, String> {
    // Placeholder for Automation Engine
    Ok("Flash process started".to_string())
}
