use tauri::command;
use std::process::Command;

#[command]
pub async fn factory_reset(serial: Option<String>, method: String) -> Result<String, String> {
    match method.as_str() {
        "fastboot" => {
            let mut cmd = Command::new("fastboot");
            if let Some(s) = serial {
                cmd.arg("-s").arg(s);
            }
            cmd.arg("-w"); // Wipe userdata and cache

            let output = cmd.output().map_err(|e| e.to_string())?;
            if output.status.success() {
                Ok("Device wiped successfully (userdata + cache)".to_string())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        },
        "recovery" => {
            // This is tricky as standard recovery doesn't always accept shell commands easily without root/twrp
            // We'll try the standard 'recovery --wipe_data' via adb shell, which works on some devices/recoveries
            let mut cmd = Command::new("adb");
            if let Some(s) = serial {
                cmd.arg("-s").arg(s);
            }
            cmd.arg("shell").arg("recovery").arg("--wipe_data");

            let output = cmd.output().map_err(|e| e.to_string())?;
            if output.status.success() {
                Ok("Wipe command sent to recovery".to_string())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        },
        _ => Err("Unknown wipe method".to_string())
    }
}
