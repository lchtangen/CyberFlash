use tauri::{command, AppHandle, Manager};
use tauri_plugin_shell::ShellExt;
use std::process::Child;
use chrono::Local;

// Keep track of the scrcpy process
#[allow(dead_code)]
struct ScrcpyState {
    process: Option<Child>,
}

#[command]
pub async fn check_scrcpy_installed(app: AppHandle) -> bool {
    let sidecar_res = app.shell().sidecar("scrcpy");
    
    if let Ok(cmd) = sidecar_res {
        if let Ok(_) = cmd.args(["--version"]).output().await {
            return true;
        }
    }
    
    // Fallback to system path
    which::which("scrcpy").is_ok()
}

#[command]
pub async fn start_scrcpy(
    app: AppHandle, 
    serial: String, 
    bitrate: u32, 
    max_fps: u32,
    record: bool,
    audio: bool
) -> Result<String, String> {
    // For now, we launch it as a separate window, but styled to look integrated
    // --window-title "CyberFlash Mirror"
    // --always-on-top
    // --window-borderless (if supported)
    
    let bitrate_str = format!("{}M", bitrate);
    let fps_str = max_fps.to_string();
    
    let mut args = vec![
        "--serial".to_string(), serial,
        "--window-title".to_string(), "CyberFlash Mirror".to_string(),
        "--always-on-top".to_string(),
        "--max-fps".to_string(), fps_str,
        "--video-bit-rate".to_string(), bitrate_str,
    ];

    if !audio {
        args.push("--no-audio".to_string());
    }

    if record {
        let download_dir = app.path().download_dir().map_err(|e| e.to_string())?;
        let recordings_dir = download_dir.join("CyberFlash").join("Recordings");
        
        if !recordings_dir.exists() {
            std::fs::create_dir_all(&recordings_dir).map_err(|e| e.to_string())?;
        }

        let filename = format!("screen_{}.mp4", Local::now().format("%Y%m%d_%H%M%S"));
        let file_path = recordings_dir.join(filename);
        
        args.push("--record".to_string());
        args.push(file_path.to_string_lossy().to_string());
    }

    // Try sidecar first, then system
    let result = app.shell().sidecar("scrcpy");
    
    if let Ok(cmd) = result {
        cmd.args(&args).spawn().map_err(|e| e.to_string())?;
    } else {
        // System command
        std::process::Command::new("scrcpy")
            .args(&args)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    Ok("Scrcpy started".to_string())
}
