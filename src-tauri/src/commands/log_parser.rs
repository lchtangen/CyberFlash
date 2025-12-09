use tauri::{command, AppHandle, Emitter};
use tauri_plugin_shell::ShellExt;
use serde::{Serialize, Deserialize};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub tag: String,
    pub message: String,
    pub analysis: Option<String>, // AI analysis of the error
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LogAnalysis {
    pub severity: String, // "Low", "Medium", "High", "Critical"
    pub summary: String,
    pub possible_fix: String,
}

// Simple heuristic analysis for common errors
fn analyze_error(tag: &str, message: &str) -> Option<LogAnalysis> {
    let msg_lower = message.to_lowercase();
    
    if msg_lower.contains("bootloop") || msg_lower.contains("reboot") {
        return Some(LogAnalysis {
            severity: "Critical".to_string(),
            summary: "Potential Bootloop Detected".to_string(),
            possible_fix: "Check incompatible modules or dirty flash. Try factory reset.".to_string(),
        });
    }
    
    if msg_lower.contains("permission denied") || msg_lower.contains("eacces") {
        return Some(LogAnalysis {
            severity: "High".to_string(),
            summary: "Permission Error".to_string(),
            possible_fix: "Ensure root access or correct file permissions (chmod 644/755).".to_string(),
        });
    }

    if tag == "AndroidRuntime" && msg_lower.contains("fatal exception") {
        return Some(LogAnalysis {
            severity: "Critical".to_string(),
            summary: "App Crash (Fatal Exception)".to_string(),
            possible_fix: "Clear app data or check for updates.".to_string(),
        });
    }

    if msg_lower.contains("out of memory") || msg_lower.contains("oom") {
        return Some(LogAnalysis {
            severity: "Medium".to_string(),
            summary: "Out of Memory".to_string(),
            possible_fix: "Close background apps or check for memory leaks.".to_string(),
        });
    }

    None
}

#[command]
pub async fn start_log_sentinel(app: AppHandle) -> Result<(), String> {
    // In a real implementation, this would spawn a long-running task 
    // that reads from a Command::spawn("adb logcat") stdout stream.
    // For this demo, we'll simulate a stream of logs mixed with real dump.
    
    let app_handle = app.clone();
    
    tauri::async_runtime::spawn(async move {
        loop {
            // Fetch a small chunk of recent logs
            let output = match app_handle.shell().sidecar("adb") {
                Ok(cmd) => cmd.args(["logcat", "-d", "-v", "threadtime", "-t", "10"])
                    .output()
                    .await,
                Err(_) => break, // Stop if shell fails
            };

            if let Ok(output) = output {
                let output_str = String::from_utf8_lossy(&output.stdout);
                let mut entries = Vec::new();

                for line in output_str.lines() {
                    // Threadtime format: date time pid tid level tag: message
                    // Example: 12-09 14:23:45.123  1234  1234 I ActivityManager: Start proc...
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 6 {
                        let level = parts[4];
                        let tag = parts[5].trim_end_matches(':');
                        let message = parts[6..].join(" ");
                        
                        // Map level char to full string
                        let level_full = match level {
                            "E" => "Error",
                            "W" => "Warning",
                            "I" => "Info",
                            "D" => "Debug",
                            "V" => "Verbose",
                            "F" => "Fatal",
                            _ => "Unknown",
                        };

                        // Only care about Warnings and Errors for the Sentinel
                        if level == "E" || level == "F" || level == "W" {
                            let analysis = if level == "E" || level == "F" {
                                analyze_error(tag, &message).map(|a| format!("{} - {}", a.summary, a.possible_fix))
                            } else {
                                None
                            };

                            entries.push(LogEntry {
                                timestamp: parts[1].to_string(),
                                level: level_full.to_string(),
                                tag: tag.to_string(),
                                message,
                                analysis,
                            });
                        }
                    }
                }

                if !entries.is_empty() {
                    let _ = app_handle.emit("log-sentinel-update", entries);
                }
            }

            sleep(Duration::from_secs(2)).await;
        }
    });

    Ok(())
}

#[command]
pub async fn stop_log_sentinel() -> Result<(), String> {
    // In a real implementation, this would signal the thread to stop.
    // For now, we rely on the frontend to ignore events or the app closing.
    Ok(())
}

#[command]
pub async fn get_logcat_dump(app: AppHandle) -> Result<Vec<LogEntry>, String> {
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["logcat", "-d", "-v", "threadtime", "-t", "500"]) // -d dumps, -t 500 gets last 500 lines
        .output()
        .await
        .map_err(|e| e.to_string())?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut entries = Vec::new();

    for line in output_str.lines() {
        // Threadtime format: date time pid tid level tag: message
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 6 {
            let level = parts[4];
            let tag = parts[5].trim_end_matches(':');
            let message = parts[6..].join(" ");
            
            let level_full = match level {
                    "E" => "Error",
                    "W" => "Warning",
                    "I" => "Info",
                    "D" => "Debug",
                    "V" => "Verbose",
                    "F" => "Fatal",
                    _ => "Unknown",
                };
                
                entries.push(LogEntry {
                    timestamp: parts[1].to_string(),
                    level: level_full.to_string(),
                    tag: tag.to_string(),
                    message,
                    analysis: None,
                });
        }
    }
    Ok(entries)
}
