use tauri::{command, AppHandle, Emitter};
use serde::{Serialize, Deserialize};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FlashConfig {
    pub name: String,
    pub device: String,
    pub version: String,
    pub steps: Vec<FlashStep>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "params")]
pub enum FlashStep {
    #[serde(rename = "wipe")]
    Wipe { partitions: Vec<String> },
    
    #[serde(rename = "flash_rom")]
    FlashRom { file: String, slot: Option<String> },
    
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
pub async fn execute_flash_plan(app: AppHandle, config: FlashConfig) -> Result<(), String> {
    let app_handle = app.clone();
    
    tauri::async_runtime::spawn(async move {
        for (index, step) in config.steps.iter().enumerate() {
            // Emit start event
            let _ = app_handle.emit("flash-plan-update", ExecutionLog {
                step_index: index,
                status: "running".to_string(),
                message: format!("Executing step {}: {:?}", index + 1, step),
            });

            // Simulate execution delay
            sleep(Duration::from_secs(2)).await;

            // In a real implementation, we would match the step type and call the respective command
            // match step {
            //     FlashStep::Wipe { partitions } => wipe_partitions(partitions).await,
            //     ...
            // }

            // Emit success event
            let _ = app_handle.emit("flash-plan-update", ExecutionLog {
                step_index: index,
                status: "success".to_string(),
                message: format!("Step {} completed successfully.", index + 1),
            });
        }
        
        let _ = app_handle.emit("flash-plan-complete", ());
    });

    Ok(())
}
