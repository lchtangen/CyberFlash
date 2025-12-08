use tauri::{command, AppHandle};
use tauri_plugin_shell::ShellExt;
use std::collections::HashMap;

#[command]
pub async fn get_device_props(app: AppHandle, serial: Option<String>) -> Result<HashMap<String, String>, String> {
    let mut cmd = app.shell().sidecar("adb").map_err(|e| e.to_string())?;
    if let Some(s) = serial {
        cmd = cmd.args(["-s", &s]);
    }
    cmd = cmd.args(["shell", "getprop"]);

    let output = cmd.output().await.map_err(|e| e.to_string())?;
    
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut props = HashMap::new();

    for line in output_str.lines() {
        // Parse lines like: [ro.product.model]: [Pixel 6]
        let parts: Vec<&str> = line.split("]: [").collect();
        if parts.len() == 2 {
            let key = parts[0].trim_start_matches('[');
            let value = parts[1].trim_end_matches(']');
            props.insert(key.to_string(), value.to_string());
        }
    }

    Ok(props)
}
