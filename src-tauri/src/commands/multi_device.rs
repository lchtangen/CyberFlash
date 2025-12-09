use tauri::{command, AppHandle, Emitter};
use tauri_plugin_shell::ShellExt;
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use tokio::task;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatchJob {
    pub id: String,
    pub device_serials: Vec<String>,
    pub action: String, // "reboot", "flash", "wipe"
    pub status: String, // "running", "completed", "failed"
    pub results: Vec<DeviceResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceResult {
    pub serial: String,
    pub success: bool,
    pub message: String,
}

async fn run_adb(app: &AppHandle, args: &[&str]) -> Result<String, String> {
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(args)
        .output()
        .await
        .map_err(|e| e.to_string())?;
        
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[command]
pub async fn execute_batch_action(app: AppHandle, serials: Vec<String>, action: String) -> Result<BatchJob, String> {
    let job_id = uuid::Uuid::new_v4().to_string();
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    for serial in serials.clone() {
        let action_clone = action.clone();
        let results_clone = results.clone();
        let app_handle = app.clone();

        let handle = task::spawn(async move {
            let (success, message) = match action_clone.as_str() {
                "reboot" => {
                    match run_adb(&app_handle, &["-s", &serial, "reboot"]).await {
                        Ok(_) => (true, "Rebooted".to_string()),
                        Err(e) => (false, e),
                    }
                },
                "reboot-bootloader" => {
                    match run_adb(&app_handle, &["-s", &serial, "reboot", "bootloader"]).await {
                        Ok(_) => (true, "Rebooted to bootloader".to_string()),
                        Err(e) => (false, e),
                    }
                },
                "reboot-recovery" => {
                    match run_adb(&app_handle, &["-s", &serial, "reboot", "recovery"]).await {
                        Ok(_) => (true, "Rebooted to recovery".to_string()),
                        Err(e) => (false, e),
                    }
                },
                _ => (false, format!("Unsupported action: {}", action_clone))
            };

            let result = DeviceResult {
                serial: serial.clone(),
                success,
                message: message.clone(),
            };

            results_clone.lock().unwrap().push(result.clone());
            
            // Emit progress event
            let _ = app_handle.emit("batch-progress", result);
        });
        handles.push(handle);
    }

    // Wait for all to finish (in a real background job we wouldn't await here if we want async return)
    // But for simplicity we await all
    for h in handles {
        let _ = h.await;
    }

    let final_results = results.lock().unwrap().clone();

    Ok(BatchJob {
        id: job_id,
        device_serials: serials,
        action,
        status: "completed".to_string(),
        results: final_results,
    })
}
