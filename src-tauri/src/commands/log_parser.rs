use tauri::{command, AppHandle, Emitter, State};
use tauri_plugin_shell::ShellExt;
use serde::{Serialize, Deserialize};
use std::time::Duration;
use tokio::time::sleep;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::fs;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ErrorPattern {
    pub pattern: String,
    pub tag: Option<String>,
    pub severity: String,
    pub summary: String,
    pub fix: String,
}

pub struct SentinelState {
    pub is_running: Arc<AtomicBool>,
    pub patterns: Arc<Mutex<Vec<ErrorPattern>>>,
}

impl Default for SentinelState {
    fn default() -> Self {
        // Load patterns from config file if possible, else use defaults
        let mut patterns = Vec::new();
        if let Ok(content) = fs::read_to_string("../config/error_patterns.json") {
             if let Ok(parsed) = serde_json::from_str(&content) {
                 patterns = parsed;
             }
        }
        
        // Fallback if file load fails (or relative path is wrong in dev vs prod)
        if patterns.is_empty() {
            patterns.push(ErrorPattern {
                pattern: "bootloop".to_string(),
                tag: None,
                severity: "Critical".to_string(),
                summary: "Potential Bootloop".to_string(),
                fix: "Factory reset recommended".to_string(),
            });
        }

        Self {
            is_running: Arc::new(AtomicBool::new(false)),
            patterns: Arc::new(Mutex::new(patterns)),
        }
    }
}

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
fn analyze_error(tag: &str, message: &str, patterns: &[ErrorPattern]) -> Option<LogAnalysis> {
    let msg_lower = message.to_lowercase();
    
    for p in patterns {
        let pattern_match = msg_lower.contains(&p.pattern) || 
                           (regex::Regex::new(&p.pattern).map(|r| r.is_match(&msg_lower)).unwrap_or(false));
        
        let tag_match = match &p.tag {
            Some(t) => tag == t,
            None => true,
        };

        if pattern_match && tag_match {
            return Some(LogAnalysis {
                severity: p.severity.clone(),
                summary: p.summary.clone(),
                possible_fix: p.fix.clone(),
            });
        }
    }

    None
}

#[command]
pub async fn start_log_sentinel(app: AppHandle, state: State<'_, SentinelState>) -> Result<(), String> {
    if state.is_running.load(Ordering::SeqCst) {
        return Ok(()); // Already running
    }
    
    state.is_running.store(true, Ordering::SeqCst);
    let is_running = state.is_running.clone();
    let patterns = state.patterns.lock().unwrap().clone(); // Clone patterns for the thread
    let app_handle = app.clone();
    
    tauri::async_runtime::spawn(async move {
        while is_running.load(Ordering::SeqCst) {
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
                                analyze_error(tag, &message, &patterns).map(|a| format!("{} - {}", a.summary, a.possible_fix))
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
pub async fn stop_log_sentinel(state: State<'_, SentinelState>) -> Result<(), String> {
    state.is_running.store(false, Ordering::SeqCst);
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

#[command]
pub async fn analyze_log_with_ai(log_entry: String, api_key: String) -> Result<String, String> {
    let prompt = format!(
        "Analyze this Android log error and suggest a fix. Be concise. Log: {}", 
        log_entry
    );
    
    crate::commands::gemini::call_gemini_api(prompt, api_key, None).await
}
