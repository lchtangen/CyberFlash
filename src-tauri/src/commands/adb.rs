use tauri::{command, AppHandle, Emitter, Manager};
use tauri_plugin_shell::ShellExt;
use crate::commands::history::HistoryDb;
use chrono::Local;

#[derive(serde::Serialize)]
pub struct FileEntry {
    name: String,
    path: String,
    is_dir: bool,
    size: String,
    permissions: String,
    date: String,
}

#[derive(serde::Serialize)]
pub struct Telemetry {
    cpu: u8,
    ram: u8,
    temp: u8,
    network: u8,
}

#[command]
pub async fn get_telemetry(app: AppHandle) -> Result<Telemetry, String> {
    // 1. Battery Temp
    let output_battery = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["shell", "dumpsys", "battery"])
        .output()
        .await
        .map_err(|e| e.to_string())?;
    
    let mut temp = 0;
    let output_str = String::from_utf8_lossy(&output_battery.stdout);
    for line in output_str.lines() {
        if line.trim().starts_with("temperature:") {
            let temp_str = line.split(':').nth(1).unwrap_or("0").trim();
            temp = temp_str.parse::<u32>().unwrap_or(0) / 10; // usually in 10th of degree C
            break;
        }
    }

    // 2. RAM
    let output_ram = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["shell", "cat", "/proc/meminfo"])
        .output()
        .await
        .map_err(|e| e.to_string())?;
    
    let mut total_ram = 1;
    let mut avail_ram = 0;
    let ram_str = String::from_utf8_lossy(&output_ram.stdout);
    for line in ram_str.lines() {
        if line.starts_with("MemTotal:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() > 1 {
                total_ram = parts[1].parse::<u64>().unwrap_or(1);
            }
        }
        if line.starts_with("MemAvailable:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() > 1 {
                avail_ram = parts[1].parse::<u64>().unwrap_or(0);
            }
        }
    }
    let ram_usage = ((total_ram - avail_ram) as f32 / total_ram as f32 * 100.0) as u8;

    // 3. CPU (Mock for now as it's slow)
    let cpu = 42; 

    // 4. Network (Mock)
    let network = 50;

    Ok(Telemetry {
        cpu,
        ram: ram_usage,
        temp: temp as u8,
        network,
    })
}

#[command]
pub async fn get_connected_devices(app: AppHandle) -> Result<Vec<String>, String> {
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["devices"])
        .output()
        .await
        .map_err(|e| e.to_string())?;
        
    let output_str = String::from_utf8_lossy(&output.stdout);
    let devices: Vec<String> = output_str
        .lines()
        .skip(1) // Skip "List of devices attached"
        .map(|line| line.split_whitespace().next().unwrap_or("").to_string())
        .filter(|s| !s.is_empty())
        .collect();
        
    Ok(devices)
}

#[command]
pub async fn kill_server(app: AppHandle) -> Result<String, String> {
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["kill-server"])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok("ADB Server killed".to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[derive(serde::Serialize, Clone)]
struct MacroEvent {
    step_type: String,
    params: serde_json::Value,
}

fn emit_macro_event(app: &AppHandle, step_type: &str, params: serde_json::Value) {
    let _ = app.emit("macro-event", MacroEvent {
        step_type: step_type.to_string(),
        params,
    });
}

#[command]
pub async fn reboot_device(app: AppHandle, mode: String) -> Result<String, String> {
    // mode: "system", "recovery", "bootloader", "sideload"
    let args = if mode == "system" {
        vec!["reboot"]
    } else if mode == "sideload" {
        vec!["reboot", "sideload-auto-reboot"]
    } else {
        vec!["reboot", &mode]
    };
    
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(&args)
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        let _ = crate::commands::history::add_activity(
            app.clone(),
            app.state::<HistoryDb>(),
            format!("Reboot Device ({})", mode),
            Some(format!("Device rebooted to {} mode via ADB", mode)),
            "success".to_string(),
            None, // device_serial is not directly available here, assuming it's handled elsewhere or not critical for this log
        ).await;
        
        // Emit Macro Event
        emit_macro_event(&app, "reboot", serde_json::json!({ "mode": mode }));

        Ok(format!("Rebooting to {}", mode))
    } else {
        let _ = crate::commands::history::add_activity(
            app.clone(),
            app.state::<HistoryDb>(),
            format!("Reboot Failed ({})", mode),
            Some(String::from_utf8_lossy(&output.stderr).to_string()),
            "error".to_string(),
            None,
        ).await;
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[command]
pub async fn adb_sideload(app: AppHandle, file_path: String) -> Result<String, String> {
    // Emit Macro Event
    emit_macro_event(&app, "sideload", serde_json::json!({ "file": file_path }));

    let (mut rx, _) = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["sideload", &file_path])
        .spawn()
        .map_err(|e| e.to_string())?;

    let mut success = false;
    while let Some(event) = rx.recv().await {
        match event {
            tauri_plugin_shell::process::CommandEvent::Stdout(line) => {
                let line_str = String::from_utf8_lossy(&line);
                let _ = app.emit("sideload-progress", line_str);
            }
            tauri_plugin_shell::process::CommandEvent::Stderr(line) => {
                 let line_str = String::from_utf8_lossy(&line);
                 let _ = app.emit("sideload-progress", line_str);
            }
            tauri_plugin_shell::process::CommandEvent::Terminated(payload) => {
                success = payload.code.unwrap_or(1) == 0;
            }
            _ => {}
        }
    }

    if success {
        let _ = crate::commands::history::add_activity(
            app.clone(),
            app.state::<HistoryDb>(),
            "ADB Sideload".to_string(),
            Some(format!("Successfully sideloaded: {}", file_path)),
            "success".to_string(),
            None,
        ).await;
        Ok("Sideload complete".to_string())
    } else {
        let _ = crate::commands::history::add_activity(
            app.clone(),
            app.state::<HistoryDb>(),
            "ADB Sideload Failed".to_string(),
            Some(format!("Failed to sideload: {}", file_path)),
            "error".to_string(),
            None,
        ).await;
        Err("Sideload failed".to_string())
    }
}



#[command]
pub async fn check_battery_level(app: AppHandle) -> Result<u32, String> {
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["shell", "dumpsys", "battery"])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    for line in output_str.lines() {
        if line.trim().starts_with("level:") {
            let level_str = line.split(':').nth(1).unwrap_or("0").trim();
            return level_str.parse::<u32>().map_err(|e| e.to_string());
        }
    }
    Err("Could not parse battery level".to_string())
}

#[command]
pub async fn list_packages(app: AppHandle) -> Result<Vec<String>, String> {
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["shell", "pm", "list", "packages"])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    let packages: Vec<String> = output_str
        .lines()
        .filter_map(|line| line.strip_prefix("package:").map(|s| s.trim().to_string()))
        .collect();
        
    Ok(packages)
}

#[command]
pub async fn install_apk(app: AppHandle, file_path: String) -> Result<String, String> {
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["install", "-r", &file_path])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        let _ = crate::commands::history::add_activity(
            app.clone(),
            app.state::<HistoryDb>(),
            "Install APK".to_string(),
            Some(format!("Installed: {}", file_path)),
            "success".to_string(),
            None,
        ).await;
        Ok("APK Installed Successfully".to_string())
    } else {
        let _ = crate::commands::history::add_activity(
            app.clone(),
            app.state::<HistoryDb>(),
            "Install APK Failed".to_string(),
            Some(format!("Failed to install {}: {}", file_path, String::from_utf8_lossy(&output.stderr))),
            "error".to_string(),
            None,
        ).await;
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[command]
pub async fn push_file(app: AppHandle, local_path: String, remote_path: String) -> Result<String, String> {
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["push", &local_path, &remote_path])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok("File pushed successfully".to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[command]
pub async fn pull_file(app: AppHandle, remote_path: String, local_path: String) -> Result<String, String> {
    let target_path = if local_path.is_empty() {
        let download_dir = app.path().download_dir().map_err(|e| e.to_string())?;
        let filename = std::path::Path::new(&remote_path).file_name().unwrap_or_default();
        download_dir.join(filename).to_string_lossy().to_string()
    } else {
        local_path
    };

    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["pull", &remote_path, &target_path])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(format!("File pulled to {}", target_path))
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[command]
pub async fn delete_file(app: AppHandle, path: String) -> Result<String, String> {
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["shell", "rm", "-rf", &path])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(format!("Deleted {}", path))
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[command]
pub async fn create_directory(app: AppHandle, path: String) -> Result<String, String> {
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["shell", "mkdir", "-p", &path])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(format!("Created directory {}", path))
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}


#[command]
pub async fn get_storage_info(app: AppHandle) -> Result<String, String> {
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["shell", "df", "-h", "/data"])
        .output()
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}



#[command]
pub async fn adb_screen_record(app: AppHandle, output_path: String, duration_sec: u32) -> Result<String, String> {
    let temp_remote_path = "/sdcard/video.mp4";
    
    // 1. Record (this blocks, so duration limit is needed)
    let _ = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["shell", "screenrecord", "--time-limit", &duration_sec.to_string(), temp_remote_path])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    // 2. Pull
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["pull", temp_remote_path, &output_path])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    // 3. Cleanup
    let _ = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["shell", "rm", temp_remote_path])
        .output()
        .await;

    if output.status.success() {
        Ok("Screen recording saved".to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[command]
pub async fn check_root_status(app: AppHandle) -> Result<bool, String> {
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["shell", "su", "-c", "id"])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    Ok(output_str.contains("uid=0(root)"))
}

#[command]
pub async fn get_display_density(app: AppHandle) -> Result<String, String> {
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["shell", "wm", "density"])
        .output()
        .await
        .map_err(|e| e.to_string())?;
    
    let output_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    // Output format: "Physical density: 480"
    Ok(output_str.replace("Physical density:", "").trim().to_string())
}

#[command]
pub async fn get_ip_address(app: AppHandle) -> Result<String, String> {
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["shell", "ip", "route"])
        .output()
        .await
        .map_err(|e| e.to_string())?;
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    // Parse logic: find line with "src", extract IP
    for line in output_str.lines() {
        if line.contains("wlan0") && line.contains("src") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            for (i, part) in parts.iter().enumerate() {
                if *part == "src" && i + 1 < parts.len() {
                    return Ok(parts[i+1].to_string());
                }
            }
        }
    }
    Err("Could not find IP address".to_string())
}

#[command]
pub async fn enable_wireless_debugging(app: AppHandle) -> Result<String, String> {
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["tcpip", "5555"])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok("Wireless debugging enabled on port 5555".to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[command]
pub async fn connect_wireless(app: AppHandle, ip: String) -> Result<String, String> {
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["connect", &format!("{}:5555", ip)])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    if output_str.contains("connected to") {
        Ok(format!("Connected to {}", ip))
    } else {
        Err(output_str.to_string())
    }
}

#[command]
pub async fn wait_for_device(app: AppHandle) -> Result<String, String> {
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["wait-for-device"])
        .output()
        .await
        .map_err(|e| e.to_string())?;
    
    if output.status.success() {
        Ok("Device found".to_string())
    } else {
        Err("Wait failed".to_string())
    }
}

#[command]
pub async fn wait_for_recovery(app: AppHandle) -> Result<String, String> {
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["wait-for-recovery"])
        .output()
        .await
        .map_err(|e| e.to_string())?;
    
    if output.status.success() {
        Ok("Recovery found".to_string())
    } else {
        Err("Wait failed".to_string())
    }
}



#[command]
pub async fn adb_backup(app: AppHandle, backup_path: String, packages: Vec<String>, apk: bool, shared: bool, system: bool) -> Result<String, String> {
    let mut args = vec!["backup".to_string(), "-f".to_string(), backup_path];
    
    if apk { args.push("-apk".to_string()); } else { args.push("-noapk".to_string()); }
    if shared { args.push("-shared".to_string()); } else { args.push("-noshared".to_string()); }
    if system { args.push("-system".to_string()); } else { args.push("-nosystem".to_string()); }
    
    if packages.is_empty() {
        args.push("-all".to_string());
    } else {
        args.extend(packages);
    }

    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(&args)
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok("Backup process finished. Verify file size.".to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[command]
pub async fn backup_partition_image(app: AppHandle, partition: String, output_name: String) -> Result<String, String> {
    // Requires Root
    let cmd = format!("dd if=/dev/block/bootdevice/by-name/{} of=/sdcard/{}.img", partition, output_name);
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["shell", "su", "-c", &cmd])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        // Pull it
        let pull_res = pull_file(app, format!("/sdcard/{}.img", output_name), format!("./backups/{}.img", output_name)).await;
        match pull_res {
            Ok(_) => Ok(format!("Partition {} backed up successfully", partition)),
            Err(e) => Err(format!("Backup created on device but pull failed: {}", e))
        }
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[command]
pub async fn remove_lock_files(app: AppHandle) -> Result<String, String> {
    // Requires Root
    // Deletes locksettings.db, *.key, etc.
    let cmd = "rm /data/system/locksettings.db* /data/system/*.key /data/system/gatekeeper.*";
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["shell", "su", "-c", cmd])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok("Lock files removed. Reboot device.".to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[command]
pub async fn enable_camera2(app: AppHandle) -> Result<String, String> {
    // Requires Root
    let cmd = "setprop persist.camera.HAL3.enabled 1";
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["shell", "su", "-c", cmd])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok("Camera2 API enabled. Reboot required.".to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[command]
pub async fn get_prop_value(app: AppHandle, key: String) -> Result<String, String> {
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["shell", "getprop", &key])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

#[command]
pub async fn set_prop_value(app: AppHandle, key: String, value: String) -> Result<String, String> {
    // Requires Root
    let cmd = format!("setprop {} {}", key, value);
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["shell", "su", "-c", &cmd])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(format!("Property {} set to {}", key, value))
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[command]
pub async fn list_third_party_apps(app: AppHandle) -> Result<Vec<String>, String> {
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["shell", "pm", "list", "packages", "-3"])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    let packages: Vec<String> = output_str
        .lines()
        .map(|line| line.replace("package:", "").trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    Ok(packages)
}

#[command]
pub async fn uninstall_package(app: AppHandle, package: String) -> Result<String, String> {
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["shell", "pm", "uninstall", "-k", "--user", "0", &package])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    if output_str.contains("Success") {
        Ok(format!("Uninstalled {}", package))
    } else {
        Err(output_str.to_string())
    }
}

#[command]
pub async fn get_battery_details(app: AppHandle) -> Result<String, String> {
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["shell", "dumpsys", "battery"])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[command]
pub async fn run_adb_shell(app: AppHandle, command: String) -> Result<String, String> {
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["shell", &command])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

pub async fn run_adb_command(app: AppHandle, args: Vec<String>) -> Result<String, String> {
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(&args)
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[command]
pub async fn take_screenshot(app: AppHandle, serial: String) -> Result<String, String> {
    let download_dir = app.path().download_dir().map_err(|e| e.to_string())?;
    let screenshots_dir = download_dir.join("CyberFlash").join("Screenshots");
    
    if !screenshots_dir.exists() {
        std::fs::create_dir_all(&screenshots_dir).map_err(|e| e.to_string())?;
    }

    let filename = format!("screenshot_{}.png", Local::now().format("%Y%m%d_%H%M%S"));
    let file_path = screenshots_dir.join(&filename);
    
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["-s", &serial, "exec-out", "screencap", "-p"])
        .output()
        .await
        .map_err(|e| e.to_string())?;
        
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    
    std::fs::write(&file_path, output.stdout).map_err(|e| e.to_string())?;
    
    Ok(file_path.to_string_lossy().to_string())
}

#[command]
pub async fn list_directory(app: AppHandle, path: String) -> Result<Vec<FileEntry>, String> {
    // Use 'ls -l' to get details. 
    // Note: Android ls format can vary (Toybox vs Busybox), but usually:
    // drwxr-xr-x  2 root root 4096 2023-01-01 12:00 foldername
    
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["shell", "ls", "-l", &path])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut entries = Vec::new();

    for line in output_str.lines() {
        if line.starts_with("total") { continue; }
        
        // Basic parsing logic
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 6 { continue; }

        // Heuristic: 
        // [0] perms (drwxr...)
        // [1] links/owner? (varies)
        // ...
        // [last] name
        // [last-1] time
        // [last-2] date
        
        let perms = parts[0];
        let is_dir = perms.starts_with('d');
        
        // Name might contain spaces, so we need to be careful.
        // Usually date/time are before the name.
        // Let's assume standard toybox ls -l:
        // perms links owner group size date time name
        
        let name_start_index = if parts.len() >= 8 { 7 } else { parts.len() - 1 };
        let name = parts[name_start_index..].join(" ");
        
        if name == "." || name == ".." { continue; }

        let size = if parts.len() > 4 { parts[4].to_string() } else { "0".to_string() };
        let date = if parts.len() > 6 { format!("{} {}", parts[5], parts[6]) } else { "".to_string() };

        let full_path = if path.ends_with('/') {
            format!("{}{}", path, name)
        } else {
            format!("{}/{}", path, name)
        };

        entries.push(FileEntry {
            name,
            path: full_path,
            is_dir,
            size,
            permissions: perms.to_string(),
            date,
        });
    }

    Ok(entries)
}
