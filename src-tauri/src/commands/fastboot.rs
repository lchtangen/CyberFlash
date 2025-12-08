use tauri::command;

#[command]
pub async fn get_fastboot_devices() -> Result<Vec<String>, String> {
    // Placeholder for Fastboot devices command
    Ok(vec![])
}
