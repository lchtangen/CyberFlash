use tauri::{command, AppHandle, Emitter};
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;
use lazy_static::lazy_static;
use crate::commands::{adb, fastboot, flash_as_code};
use std::fs;

// Global state for Zero Touch
lazy_static! {
    static ref ZERO_TOUCH_STATE: Arc<Mutex<ZeroTouchState>> = Arc::new(Mutex::new(ZeroTouchState::default()));
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ZeroTouchState {
    pub is_enabled: bool,
    pub target_device_serial: Option<String>,
    pub profile_path: Option<String>,
    pub auto_start_delay: u64, // Seconds
    pub processed_devices: Vec<String>, // Serials that have been handled
    pub is_counting_down: bool,
}

#[command]
pub fn get_zero_touch_state() -> ZeroTouchState {
    ZERO_TOUCH_STATE.lock().unwrap().clone()
}

#[command]
pub fn set_zero_touch_state(state: ZeroTouchState) {
    let mut s = ZERO_TOUCH_STATE.lock().unwrap();
    // Preserve processed_devices and running state when updating config
    let old_processed = s.processed_devices.clone();
    let old_counting = s.is_counting_down;
    
    *s = state;
    s.processed_devices = old_processed;
    s.is_counting_down = old_counting;
}

#[command]
pub async fn start_zero_touch_service(app: AppHandle) -> Result<(), String> {
    let app_handle = app.clone();
    
    tauri::async_runtime::spawn(async move {
        loop {
            sleep(Duration::from_secs(3)).await;

            let (target_serial, profile_path, processed, is_enabled, is_counting_down) = {
                let state_guard = ZERO_TOUCH_STATE.lock().unwrap();
                (
                    state_guard.target_device_serial.clone(),
                    state_guard.profile_path.clone(),
                    state_guard.processed_devices.clone(),
                    state_guard.is_enabled,
                    state_guard.is_counting_down
                )
            };

            if !is_enabled || is_counting_down {
                continue;
            }
            
            // Check ADB devices
            if let Ok(devices) = adb::get_connected_devices(app_handle.clone()).await {
                for serial in devices {
                    process_device(app_handle.clone(), serial, &target_serial, &profile_path, &processed).await;
                }
            }

            // Check Fastboot devices
            if let Ok(devices) = fastboot::get_fastboot_devices(app_handle.clone()).await {
                for serial in devices {
                    process_device(app_handle.clone(), serial, &target_serial, &profile_path, &processed).await;
                }
            }
        }
    });
    
    Ok(())
}async fn process_device(
    app: AppHandle, 
    serial: String, 
    target_serial: &Option<String>, 
    profile_path: &Option<String>,
    processed: &[String]
) {
    // 1. Check if already processed
    if processed.contains(&serial) {
        return;
    }

    // 2. Check if matches target (if set)
    if let Some(target) = target_serial {
        if !target.is_empty() && target != &serial {
            return;
        }
    }

    // 3. Check if profile exists
    let path = match profile_path {
        Some(p) if !p.is_empty() => p,
        _ => return,
    };

    // 4. Trigger Countdown
    let delay = {
        let mut state = ZERO_TOUCH_STATE.lock().unwrap();
        state.is_counting_down = true;
        state.auto_start_delay
    };

    let _ = app.emit("zero-touch-detected", serial.clone());

    // Countdown loop
    for i in (0..delay).rev() {
        let _ = app.emit("zero-touch-countdown", i);
        sleep(Duration::from_secs(1)).await;
        
        // Check if cancelled
        let is_cancelled = !ZERO_TOUCH_STATE.lock().unwrap().is_counting_down;
        if is_cancelled {
            let _ = app.emit("zero-touch-cancelled", ());
            return;
        }
    }

    // 5. Execute
    let _ = app.emit("zero-touch-started", serial.clone());
    
    // Mark as processed
    {
        let mut state = ZERO_TOUCH_STATE.lock().unwrap();
        state.processed_devices.push(serial.clone());
        state.is_counting_down = false;
    }

    // Load and Run Profile
    match fs::read_to_string(path) {
        Ok(content) => {
            match flash_as_code::validate_flash_config(content).await {
                Ok(config) => {
                    let _ = flash_as_code::execute_flash_plan(app.clone(), config).await;
                },
                Err(e) => { let _ = app.emit("zero-touch-error", e); }
            }
        },
        Err(e) => { let _ = app.emit("zero-touch-error", e.to_string()); }
    }
}

#[command]
pub fn cancel_zero_touch() {
    let mut state = ZERO_TOUCH_STATE.lock().unwrap();
    state.is_counting_down = false;
}

#[command]
pub async fn check_zero_touch_trigger(_app: AppHandle, _device_serial: String) -> Result<bool, String> {
    // Legacy function kept for compatibility, but logic moved to service
    Ok(false)
}
