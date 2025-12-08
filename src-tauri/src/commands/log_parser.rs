use tauri::{command, AppHandle};
use tauri_plugin_shell::ShellExt;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct LogEntry {
    level: String,
    tag: String,
    message: String,
}

#[command]
pub async fn get_logcat_dump(app: AppHandle) -> Result<Vec<LogEntry>, String> {
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["logcat", "-d", "-v", "brief", "-t", "500"]) // -d dumps, -t 500 gets last 500 lines
        .output()
        .await
        .map_err(|e| e.to_string())?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut entries = Vec::new();

    for line in output_str.lines() {
        // Brief format: I/Tag(PID): Message
        if let Some((header, msg)) = line.split_once(':') {
            let header = header.trim();
            if header.len() > 2 {
                let level_char = &header[0..1];
                let level = match level_char {
                     "E" => "Error",
                     "W" => "Warning",
                     "I" => "Info",
                     "D" => "Debug",
                     "V" => "Verbose",
                     "F" => "Fatal",
                     _ => "Unknown",
                 };
                 
                 // Extract Tag from "I/Tag(PID)"
                 // Skip "I/" (2 chars)
                 let rest = &header[2..];
                 let tag = rest.split('(').next().unwrap_or(rest).trim();
                 
                 entries.push(LogEntry {
                     level: level.to_string(),
                     tag: tag.to_string(),
                     message: msg.trim().to_string(),
                 });
            }
        }
    }
    
    Ok(entries)
}
