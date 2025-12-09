use tauri::command;
use serde::Serialize;
use std::path::Path;
use std::env;

#[derive(Debug, Serialize)]
pub struct DriverStatus {
    pub platform: String,
    pub adb_installed: bool,
    pub fastboot_installed: bool,
    pub udev_rules_ok: bool,
    pub drivers_ok: bool,
    pub message: String,
}

#[command]
pub async fn check_drivers() -> Result<DriverStatus, String> {
    let platform = env::consts::OS.to_string();
    let mut adb_installed = false;
    let mut fastboot_installed = false;
    let mut udev_rules_ok = true; // Default to true for non-Linux
    let mut drivers_ok = true; // Default to true for non-Windows
    let mut message = String::new();

    // Check for binaries
    if let Ok(_) = which::which("adb") {
        adb_installed = true;
    }
    if let Ok(_) = which::which("fastboot") {
        fastboot_installed = true;
    }

    // Platform specific checks
    if platform == "linux" {
        let rules_path = Path::new("/etc/udev/rules.d/51-android.rules");
        if !rules_path.exists() {
            udev_rules_ok = false;
            message.push_str("Missing udev rules for Android devices. ");
        }
    } else if platform == "windows" {
        // Simple check: if we can't see any devices but binaries exist, might be driver issue
        // This is a heuristic. Real driver check is complex.
        drivers_ok = true; // Placeholder for complex Windows driver check
    }

    if !adb_installed || !fastboot_installed {
        message.push_str("ADB or Fastboot binaries not found in PATH. ");
    }

    if message.is_empty() {
        message = "All systems nominal.".to_string();
    }

    Ok(DriverStatus {
        platform,
        adb_installed,
        fastboot_installed,
        udev_rules_ok,
        drivers_ok,
        message,
    })
}

#[command]
pub async fn fix_drivers() -> Result<String, String> {
    let platform = env::consts::OS;

    match platform {
        "linux" => {
            // On Linux, we can't easily run sudo commands from the GUI without a prompt.
            // Best practice: Return a command string for the user to run, or try pkexec if available.
            // For safety and simplicity in this V2, we'll generate a script command.
            
            let rules_url = "https://raw.githubusercontent.com/M0Rf30/android-udev-rules/master/51-android.rules";
            let cmd = format!(
                "sudo wget -O /etc/udev/rules.d/51-android.rules {} && sudo udevadm control --reload-rules && sudo udevadm trigger",
                rules_url
            );
            
            // We return a special prefix so frontend knows to display it as a code block to copy
            Ok(format!("LINUX_CMD:{}", cmd))
        },
        "windows" => {
            // On Windows, we could trigger a download of the Universal ADB Driver
            // For now, return a message
            Ok("Please download and install the Universal ADB Driver from: https://adb.clockworkmod.com/".to_string())
        },
        "macos" => {
            Ok("On macOS, please install android-platform-tools via Homebrew: brew install android-platform-tools".to_string())
        },
        _ => Err("Unsupported platform for auto-fix.".to_string()),
    }
}

#[command]
pub async fn check_internet_connection() -> bool {
    use std::net::TcpStream;
    // Simple check: try to connect to Google DNS
    TcpStream::connect("8.8.8.8:53").is_ok()
}
