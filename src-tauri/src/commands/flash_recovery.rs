use tauri::command;
use std::process::Command;

#[command]
pub async fn flash_recovery(path: String, slot: String, serial: Option<String>) -> Result<String, String> {
    // slot can be "all", "a", "b", or "current"
    // If "all", we flash recovery_a and recovery_b
    // If "current", we just flash "recovery" (letting fastboot decide or if non-A/B)
    
    let targets = if slot == "all" {
        vec!["recovery_a", "recovery_b"]
    } else if slot == "a" {
        vec!["recovery_a"]
    } else if slot == "b" {
        vec!["recovery_b"]
    } else {
        vec!["recovery"]
    };

    for target in targets {
        let mut cmd = Command::new("fastboot");
        if let Some(ref s) = serial {
            cmd.arg("-s").arg(s);
        }
        cmd.arg("flash").arg(target).arg(&path);

        let output = cmd.output().map_err(|e| e.to_string())?;
        if !output.status.success() {
            return Err(format!("Failed to flash {}: {}", target, String::from_utf8_lossy(&output.stderr)));
        }
    }

    Ok("Recovery flashed successfully".to_string())
}

#[command]
pub async fn boot_recovery(path: String, serial: Option<String>) -> Result<String, String> {
    let mut cmd = Command::new("fastboot");
    if let Some(s) = serial {
        cmd.arg("-s").arg(s);
    }
    cmd.arg("boot").arg(&path);

    let output = cmd.output().map_err(|e| e.to_string())?;
    if output.status.success() {
        Ok("Booting recovery image...".to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
