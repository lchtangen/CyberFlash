use tauri::{command, AppHandle, Emitter};
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
            // Simulate action execution
            // In real app, call adb/fastboot commands here
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            
            let success = true; // Mock success
            let message = format!("Action {} completed on {}", action_clone, serial);

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
