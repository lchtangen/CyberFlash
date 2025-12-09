use tauri::{command, AppHandle, Manager};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;
use crate::commands::flash_as_code::{FlashConfig, execute_flash_plan, FlashState};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScheduledTask {
    pub device_serial: String,
    pub workflow_file: String,
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScheduleConfig {
    pub tasks: Vec<ScheduledTask>,
}

pub async fn check_and_run_schedules(app: AppHandle, serial: String) {
    // Load schedule config
    let config_path = app.path().app_config_dir().unwrap_or(PathBuf::from("config")).join("schedules.json");
    
    if !config_path.exists() {
        return;
    }

    let content = match fs::read_to_string(&config_path) {
        Ok(c) => c,
        Err(_) => return,
    };

    let config: ScheduleConfig = match serde_json::from_str(&content) {
        Ok(c) => c,
        Err(_) => return,
    };

    for task in config.tasks {
        if task.enabled && (task.device_serial == "*" || task.device_serial == serial) {
            // Found a matching task!
            println!("Found scheduled task for device {}: {}", serial, task.workflow_file);
            
            // Load the workflow file
            let workflow_path = app.path().app_config_dir().unwrap_or(PathBuf::from("config")).join("workflows").join(&task.workflow_file);
            if let Ok(yaml_content) = fs::read_to_string(workflow_path) {
                if let Ok(flash_config) = serde_yaml::from_str::<FlashConfig>(&yaml_content) {
                    // Execute it!
                    let _ = execute_flash_plan(app.clone(), app.state::<FlashState>(), flash_config).await;
                }
            }
        }
    }
}

#[command]
pub fn save_schedule(app: AppHandle, task: ScheduledTask) -> Result<(), String> {
    let config_path = app.path().app_config_dir().unwrap_or(PathBuf::from("config")).join("schedules.json");
    
    let mut config = if config_path.exists() {
        let content = fs::read_to_string(&config_path).map_err(|e| e.to_string())?;
        serde_json::from_str::<ScheduleConfig>(&content).unwrap_or(ScheduleConfig { tasks: vec![] })
    } else {
        ScheduleConfig { tasks: vec![] }
    };

    config.tasks.push(task);

    let json = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    fs::write(config_path, json).map_err(|e| e.to_string())?;

    Ok(())
}
