use tauri::command;
use std::process::Command;

#[command]
pub fn check_permissions() -> Result<bool, String> {
    #[cfg(target_os = "linux")]
    {
        let output = Command::new("groups").output().map_err(|e| e.to_string())?;
        let output_str = String::from_utf8_lossy(&output.stdout);
        if output_str.contains("plugdev") {
            return Ok(true);
        } else {
            // Also check if udev rules exist
            let rules_path = std::path::Path::new("/etc/udev/rules.d/51-android.rules");
            if rules_path.exists() {
                return Ok(true);
            }
            return Ok(false);
        }
    }
    #[cfg(not(target_os = "linux"))]
    Ok(true)
}

#[command]
pub fn request_permissions() -> Result<String, String> {
    #[cfg(target_os = "linux")]
    {
        return Ok("Please run 'sudo usermod -aG plugdev $USER' and log out/in, or install udev rules.".to_string());
    }
    #[cfg(not(target_os = "linux"))]
    Ok("Permissions granted".to_string())
}
