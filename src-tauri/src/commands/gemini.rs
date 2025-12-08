use tauri::command;

#[command]
pub async fn ask_gemini(prompt: String) -> Result<String, String> {
    // Placeholder for Gemini AI integration
    Ok(format!("Gemini says: {}", prompt))
}
