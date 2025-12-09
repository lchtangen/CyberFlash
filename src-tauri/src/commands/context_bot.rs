use tauri::command;
use serde::Serialize;

#[derive(Serialize)]
pub struct ContextHelp {
    pub title: String,
    pub explanation: String,
    pub suggestion: String,
    pub confidence: u8,
}

#[command]
pub async fn analyze_error_context(error_message: String, _current_view: String) -> Result<ContextHelp, String> {
    let err_lower = error_message.to_lowercase();
    
    // 1. Status 7 (Assert Failed)
    if err_lower.contains("status 7") || err_lower.contains("assert failed") {
        return Ok(ContextHelp {
            title: "Updater Script Assertion Failed (Status 7)".to_string(),
            explanation: "The ROM you are trying to flash checks your device model or recovery version, and the check failed.".to_string(),
            suggestion: "1. Update your Recovery (TWRP/OrangeFox) to the latest version.\n2. Ensure you downloaded the correct ROM for your specific device codename.\n3. (Advanced) Remove the 'assert' lines from the updater-script.".to_string(),
            confidence: 95,
        });
    }

    // 2. Status 1 (Binary Error)
    if err_lower.contains("status 1") {
        return Ok(ContextHelp {
            title: "Generic Binary Error (Status 1)".to_string(),
            explanation: "The installer binary crashed. This is often caused by firmware mismatch or corrupted zip file.".to_string(),
            suggestion: "1. Verify the MD5/SHA256 of the ROM zip.\n2. Ensure you are on the required firmware (e.g., OOS 11 before flashing OOS 12 ROMs).".to_string(),
            confidence: 90,
        });
    }

    // 3. ADB Device Not Found
    if err_lower.contains("device not found") || err_lower.contains("no devices/emulators") {
        return Ok(ContextHelp {
            title: "Device Not Detected".to_string(),
            explanation: "The computer cannot communicate with your phone via ADB/Fastboot.".to_string(),
            suggestion: "1. Check your USB cable (try a different port).\n2. Install/Update USB Drivers (use the Auto-Driver Fetch tool).\n3. Toggle USB Debugging off and on.".to_string(),
            confidence: 98,
        });
    }

    // 4. Fastboot Waiting
    if err_lower.contains("waiting for device") || err_lower.contains("waiting for any device") {
        return Ok(ContextHelp {
            title: "Stuck Waiting for Device".to_string(),
            explanation: "Fastboot command is waiting for a connection.".to_string(),
            suggestion: "1. Ensure the device is actually in Fastboot/Bootloader mode.\n2. Linux users: Check udev rules.\n3. Windows users: Check Device Manager for 'Android Bootloader Interface'.".to_string(),
            confidence: 95,
        });
    }

    // 5. Bootloop
    if err_lower.contains("bootloop") || err_lower.contains("stuck at logo") {
        return Ok(ContextHelp {
            title: "Bootloop Detected".to_string(),
            explanation: "The system cannot finish booting, likely due to incompatible data or bad kernel.".to_string(),
            suggestion: "1. Boot into Recovery and 'Format Data' (Factory Reset).\n2. If you just flashed Magisk, try flashing the uninstall zip.\n3. Flash the stock boot.img.".to_string(),
            confidence: 90,
        });
    }

    // 6. Permission Denied
    if err_lower.contains("permission denied") || err_lower.contains("unauthorized") {
        return Ok(ContextHelp {
            title: "ADB Unauthorized".to_string(),
            explanation: "The device has not authorized this computer for debugging.".to_string(),
            suggestion: "Check your phone screen for a popup asking to 'Allow USB Debugging' and tap Allow.".to_string(),
            confidence: 99,
        });
    }

    // Fallback for unknown errors
    Ok(ContextHelp {
        title: "Unknown Error".to_string(),
        explanation: format!("I don't have a specific fix for '{}' in my local database.", error_message),
        suggestion: "Click 'Ask AI' to send this error to Gemini for a deeper analysis.".to_string(),
        confidence: 20,
    })
}
