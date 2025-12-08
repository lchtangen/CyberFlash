use tauri::command;
use std::path::Path;

#[derive(serde::Serialize)]
pub struct DriverStatus {
    pub platform: String,
    pub adb_installed: bool,
    pub fastboot_installed: bool,
    pub udev_rules_ok: bool, // Linux only
    pub drivers_ok: bool,    // Windows only
    pub message: String,
}

#[command]
pub async fn check_drivers() -> Result<DriverStatus, String> {
    let platform = std::env::consts::OS.to_string();
    let mut status = DriverStatus {
        platform: platform.clone(),
        adb_installed: false,
        fastboot_installed: false,
        udev_rules_ok: true, // Default true for non-linux
        drivers_ok: true,    // Default true for non-windows
        message: "Checking system...".to_string(),
    };

    // Check ADB
    if let Ok(_) = which::which("adb") {
        status.adb_installed = true;
    }

    // Check Fastboot
    if let Ok(_) = which::which("fastboot") {
        status.fastboot_installed = true;
    }

    if platform == "linux" {
        // Check udev rules
        // Common path: /etc/udev/rules.d/51-android.rules
        let rules_path = Path::new("/etc/udev/rules.d/51-android.rules");
        if !rules_path.exists() {
            status.udev_rules_ok = false;
            status.message = "Missing udev rules for Android devices.".to_string();
        } else {
            status.message = "System appears ready.".to_string();
        }
    } else if platform == "windows" {
        // Windows logic (placeholder)
        status.message = "Ensure OnePlus drivers are installed.".to_string();
    } else if platform == "macos" {
        // macOS logic
        status.message = "Ensure Terminal has USB permissions.".to_string();
    }

    if !status.adb_installed || !status.fastboot_installed {
        status.message = "ADB or Fastboot binaries not found in PATH.".to_string();
    }

    Ok(status)
}

#[command]
pub async fn check_internet_connection() -> Result<bool, String> {
    // Simple ping to google DNS
    match reqwest::get("https://8.8.8.8").await {
        Ok(_) => Ok(true),
        Err(_) => Ok(false), // Or try another host
    }
}
