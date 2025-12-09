use tauri::{command, AppHandle, Emitter, State};
use serde::{Serialize, Deserialize};
use std::time::Duration;
use std::collections::HashMap;
use tokio::time::sleep;
use chrono::Local;
use crate::commands::{adb, fastboot};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct FlashState {
    pub is_cancelled: AtomicBool,
    pub is_paused: AtomicBool,
}

impl Default for FlashState {
    fn default() -> Self {
        Self {
            is_cancelled: AtomicBool::new(false),
            is_paused: AtomicBool::new(false),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FlashConfig {
    pub name: String,
    pub device: String,
    pub version: String,
    #[serde(default)]
    pub continue_on_error: bool,
    pub steps: Vec<FlashStep>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "params")]
pub enum FlashStep {
    #[serde(rename = "wipe")]
    Wipe { partitions: Vec<String> },
    
    #[serde(rename = "flash_image")]
    FlashImage { partition: String, file: String, slot: Option<String> },

    #[serde(rename = "flash_zip")]
    FlashZip { file: String }, // fastboot update
    
    #[serde(rename = "flash_recovery")]
    FlashRecovery { file: String },
    
    #[serde(rename = "sideload")]
    Sideload { file: String },
    
    #[serde(rename = "reboot")]
    Reboot { mode: String }, // "system", "recovery", "bootloader"
    
    #[serde(rename = "wait")]
    Wait { seconds: u64 },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExecutionLog {
    pub step_index: usize,
    pub status: String, // "pending", "running", "success", "error"
    pub message: String,
}

#[command]
pub async fn validate_flash_config(yaml_content: String) -> Result<FlashConfig, String> {
    let config: FlashConfig = serde_yaml::from_str(&yaml_content)
        .map_err(|e| format!("Invalid YAML format: {}", e))?;
        
    // Basic validation logic
    if config.steps.is_empty() {
        return Err("Configuration must contain at least one step.".to_string());
    }

    Ok(config)
}

#[command]
pub async fn cancel_flash_process(state: State<'_, FlashState>) -> Result<(), String> {
    state.is_cancelled.store(true, Ordering::SeqCst);
    Ok(())
}

#[command]
pub async fn pause_flash_process(state: State<'_, FlashState>) -> Result<(), String> {
    state.is_paused.store(true, Ordering::SeqCst);
    Ok(())
}

#[command]
pub async fn resume_flash_process(state: State<'_, FlashState>) -> Result<(), String> {
    state.is_paused.store(false, Ordering::SeqCst);
    Ok(())
}

#[command]
pub async fn convert_script_to_config(script_content: String, device: String) -> Result<FlashConfig, String> {
    let mut steps = Vec::new();

    // Reset cancellation state
    // This function is for converting, not executing, so these lines are out of place.
    // state.is_cancelled.store(false, Ordering::SeqCst);
    // state.is_paused.store(false, Ordering::SeqCst);
    
    // Initialize variables
    for line in script_content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("REM") || trimmed.starts_with("::") {
            continue;
        }
        
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.is_empty() { continue; }
        
        // Basic parsing for fastboot commands
        if parts[0] == "fastboot" {
            if parts.len() < 2 { continue; }
            match parts[1] {
                "flash" => {
                    if parts.len() >= 4 {
                        steps.push(FlashStep::FlashImage {
                            partition: parts[2].to_string(),
                            file: parts[3].to_string(),
                            slot: None, // TODO: Handle slot logic if present
                        });
                    }
                },
                "erase" => {
                    if parts.len() >= 3 {
                        steps.push(FlashStep::Wipe {
                            partitions: vec![parts[2].to_string()],
                        });
                    }
                },
                "reboot" => {
                    let mode = if parts.len() > 2 { parts[2].to_string() } else { "system".to_string() };
                    steps.push(FlashStep::Reboot { mode });
                },
                "-w" => {
                    steps.push(FlashStep::Wipe { partitions: vec!["userdata".to_string(), "cache".to_string()] });
                },
                "update" => {
                    if parts.len() >= 3 {
                        steps.push(FlashStep::FlashZip { file: parts[2].to_string() });
                    }
                },
                _ => {}
            }
        } else if parts[0] == "sleep" || parts[0] == "timeout" {
             if parts.len() >= 2 {
                 if let Ok(secs) = parts[1].parse::<u64>() {
                     steps.push(FlashStep::Wait { seconds: secs });
                 }
             }
        }
    }
    
    if steps.is_empty() {
        return Err("No valid flash steps found in script".to_string());
    }
    
    Ok(FlashConfig {
        name: "Imported Script".to_string(),
        device,
        version: "1.0".to_string(),
        continue_on_error: false,
        steps,
    })
}

fn substitute_variables(text: &str, vars: &HashMap<String, String>) -> String {
    let mut result = text.to_string();
    for (key, value) in vars {
        result = result.replace(&format!("${}", key), value);
    }
    result
}

#[command]
pub async fn execute_flash_plan(app: AppHandle, state: State<'_, FlashState>, config: FlashConfig) -> Result<(), String> {
    let app_handle = app.clone();
    
    // Reset cancellation state
    state.is_cancelled.store(false, Ordering::SeqCst);
    
    // Initialize variables
    let mut vars = HashMap::new();
    vars.insert("DATE".to_string(), Local::now().format("%Y-%m-%d").to_string());
    vars.insert("TIME".to_string(), Local::now().format("%H:%M:%S").to_string());
    vars.insert("DEVICE_SERIAL".to_string(), config.device.clone()); // Default to config device
    
    // If config.device is "auto", try to detect
    if config.device == "auto" {
        if let Ok(devices) = fastboot::get_fastboot_devices(app.clone()).await {
            if let Some(first) = devices.first() {
                vars.insert("DEVICE_SERIAL".to_string(), first.clone());
            }
        }
    }
    for (index, step) in config.steps.iter().enumerate() {
        // Check for cancellation
        if state.is_cancelled.load(Ordering::SeqCst) {
            let _ = app_handle.emit("flash-plan-update", ExecutionLog {
                step_index: index,
                status: "error".to_string(),
                message: "Flash process cancelled by user.".to_string(),
            });
            return Err("Flash process cancelled by user.".to_string());
        }

        // Check for pause
        while state.is_paused.load(Ordering::SeqCst) {
            // Check cancellation while paused
            if state.is_cancelled.load(Ordering::SeqCst) {
                 let _ = app_handle.emit("flash-plan-update", ExecutionLog {
                    step_index: index,
                    status: "error".to_string(),
                    message: "Flash process cancelled by user.".to_string(),
                });
                return Err("Flash process cancelled by user.".to_string());
            }
            
            let _ = app_handle.emit("flash-plan-update", ExecutionLog {
                step_index: index,
                status: "paused".to_string(),
                message: "Process paused. Waiting for resume...".to_string(),
            });
            sleep(Duration::from_millis(500)).await;
        }

        // Emit start event
        let _ = app_handle.emit("flash-plan-update", ExecutionLog {
            step_index: index,
            status: "running".to_string(),
            message: format!("Executing step {}: {:?}", index + 1, step),
        });

        let result = match step {
            FlashStep::Wait { seconds } => {
                sleep(Duration::from_secs(*seconds)).await;
                Ok("Waited".to_string())
            },
            FlashStep::Reboot { mode } => {
                let mode_sub = substitute_variables(mode, &vars);
                let res = adb::reboot_device(app_handle.clone(), mode_sub.clone()).await;
                if res.is_err() {
                    fastboot::reboot(app_handle.clone(), Some(mode_sub), None).await
                } else {
                    res
                }
            },
            FlashStep::Wipe { partitions } => {
                let mut last_res = Ok("Wiped".to_string());
                for part in partitions {
                    let part_sub = substitute_variables(part, &vars);
                    last_res = fastboot::erase_partition(app_handle.clone(), part_sub, None).await;
                    if last_res.is_err() { break; }
                }
                last_res
            },
            FlashStep::FlashImage { partition, file, slot } => {
                let part_sub = substitute_variables(partition, &vars);
                let file_sub = substitute_variables(file, &vars);
                
                let mut target_partition = part_sub.clone();
                if let Some(s) = slot {
                    let slot_sub = substitute_variables(s, &vars);
                    target_partition = format!("{}_{}", part_sub, slot_sub);
                }
                fastboot::flash_partition_stream(app_handle.clone(), target_partition, file_sub, None).await
            },
            FlashStep::FlashZip { file } => {
                let file_sub = substitute_variables(file, &vars);
                // fastboot update <zip>
                fastboot::run_fastboot_cmd(app_handle.clone(), vec!["update", &file_sub]).await
            },
            FlashStep::FlashRecovery { file } => {
                let file_sub = substitute_variables(file, &vars);
                fastboot::flash_partition_stream(app_handle.clone(), "recovery".to_string(), file_sub, None).await
            },
            FlashStep::Sideload { file } => {
                let file_sub = substitute_variables(file, &vars);
                adb::adb_sideload(app_handle.clone(), file_sub).await
            },
        };

        match result {
            Ok(msg) => {
                let _ = app_handle.emit("flash-plan-update", ExecutionLog {
                    step_index: index,
                    status: "success".to_string(),
                    message: msg,
                });
            },
            Err(e) => {
                let _ = app_handle.emit("flash-plan-update", ExecutionLog {
                    step_index: index,
                    status: "error".to_string(),
                    message: e.clone(),
                });
                
                if !config.continue_on_error {
                    return Err(e); // Stop execution on error
                }
            }
        }
    }
    
    let _ = app_handle.emit("flash-plan-complete", ());

    Ok(())
}
