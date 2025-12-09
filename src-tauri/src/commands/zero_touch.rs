use tauri::{command, AppHandle, Emitter};
use serde::{Serialize, Deserialize};
use std::sync::Mutex;
use lazy_static::lazy_static;

// Global state for Zero Touch
lazy_static! {
    static ref ZERO_TOUCH_STATE: Mutex<ZeroTouchState> = Mutex::new(ZeroTouchState::default());
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ZeroTouchState {
    pub is_enabled: bool,
    pub target_device_serial: Option<String>,
    pub profile_path: Option<String>,
    pub auto_start_delay: u64, // Seconds
}

#[command]
pub fn get_zero_touch_state() -> ZeroTouchState {
    ZERO_TOUCH_STATE.lock().unwrap().clone()
}

#[command]
pub fn set_zero_touch_state(state: ZeroTouchState) {
    let mut s = ZERO_TOUCH_STATE.lock().unwrap();
    *s = state;
}

#[command]
pub async fn check_zero_touch_trigger(app: AppHandle, device_serial: String) -> Result<bool, String> {
    let state = ZERO_TOUCH_STATE.lock().unwrap().clone();
    
    if !state.is_enabled {
        return Ok(false);
    }

    // If a specific serial is targeted, check it
    if let Some(target) = &state.target_device_serial {
        if target != &device_serial {
            return Ok(false);
        }
    }

    // If we have a profile, trigger the countdown
    if let Some(_profile) = &state.profile_path {
        // Emit event to frontend to show countdown overlay
        app.emit("zero-touch-countdown", state.auto_start_delay)
            .map_err(|e| e.to_string())?;
            
        return Ok(true);
    }

    Ok(false)
}
