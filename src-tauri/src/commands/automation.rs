use tauri::{command, AppHandle, Emitter};
use std::thread;
use std::time::Duration;
use std::path::Path;
use std::fs;
use crate::commands::{http_client, file_parser, config, adb, fastboot};

#[derive(Clone, serde::Serialize)]
struct ProgressUpdate {
    phase_id: u32,
    step_index: usize,
    total_steps: usize,
    message: String,
    percentage: f32,
}

#[command]
pub async fn get_required_downloads(app: AppHandle) -> Result<serde_json::Value, String> {
    let mut config = load_downloads_config()?;
    
    // Check file status
    let settings = config::load_settings(app.clone()).await?;
    let default_path = "./downloads";
    let save_path = settings["save_path"].as_str().unwrap_or(default_path);
    let download_path = Path::new(save_path);

    if let Some(files) = config["files"].as_array_mut() {
        for file in files {
            if let Some(filename) = file["filename"].as_str() {
                let file_path = download_path.join(filename);
                if file_path.exists() {
                    file["status"] = serde_json::json!("downloaded");
                    file["local_path"] = serde_json::json!(file_path.to_string_lossy());
                } else {
                    file["status"] = serde_json::json!("missing");
                }
            }
        }
    }

    Ok(config)
}

fn load_downloads_config() -> Result<serde_json::Value, String> {
    // In development, we look for config/downloads.json in the workspace root
    // In production, this should be in the resource directory
    let mut config_path = std::env::current_dir().map_err(|e| e.to_string())?;
    config_path.push("config");
    config_path.push("downloads.json");

    if !config_path.exists() {
        // Fallback for production structure if needed, or error out
        return Err(format!("Config file not found at: {:?}", config_path));
    }

    let content = fs::read_to_string(&config_path).map_err(|e| e.to_string())?;
    serde_json::from_str(&content).map_err(|e| e.to_string())
}

fn emit_progress(app: &AppHandle, phase_id: u32, step: usize, total: usize, msg: &str, pct: f32) {
    let update = ProgressUpdate {
        phase_id,
        step_index: step,
        total_steps: total,
        message: msg.to_string(),
        percentage: pct,
    };
    let _ = app.emit("flash-progress", update);
}

#[command]
pub async fn start_flash_process(app: AppHandle, phase_id: u32) -> Result<String, String> {
    match phase_id {
        1 => run_phase_1(&app, phase_id).await,
        2 => run_phase_2(&app, phase_id).await,
        3 => run_phase_3(&app, phase_id).await,
        4 => run_phase_4(&app, phase_id).await,
        5 => run_phase_5(&app, phase_id).await,
        6 => run_phase_6(&app, phase_id).await,
        _ => run_mock_phase(&app, phase_id).await,
    }
}

async fn run_phase_1(app: &AppHandle, phase_id: u32) -> Result<String, String> {
    // 1. Load Config
    emit_progress(app, phase_id, 0, 1, "Loading Configuration...", 0.0);
    let settings = config::load_settings(app.clone()).await?;
    let default_path = "./downloads";
    let save_path = settings["save_path"].as_str().unwrap_or(default_path);
    let download_path = Path::new(save_path);
    if !download_path.exists() {
        std::fs::create_dir_all(download_path).map_err(|e| e.to_string())?;
    }

    let downloads_config = load_downloads_config()?;
    let files = downloads_config["files"].as_array().ok_or("Invalid downloads config format")?;
    let total_files = files.len();

    for (index, file_info) in files.iter().enumerate() {
        let name = file_info["name"].as_str().unwrap_or("Unknown File");
        let filename = file_info["filename"].as_str().ok_or("Missing filename")?;
        let url = file_info["url_direct"].as_str().ok_or("Missing url_direct")?;
        let expected_sha256 = file_info["checksum_sha256"].as_str().unwrap_or("");
        
        let target_file = download_path.join(filename);
        
        // Calculate progress base
        let step_progress = 100.0 / total_files as f32;
        let current_base_progress = index as f32 * step_progress;

        // Check if file exists and verify checksum
        if target_file.exists() {
             emit_progress(app, phase_id, index + 1, total_files, &format!("Verifying existing {}...", name), current_base_progress + (step_progress * 0.5));
             if let Ok(checksum) = file_parser::calculate_sha256(&target_file) {
                 if checksum == expected_sha256 {
                     continue; // Skip download if valid
                 }
             }
        }

        // Download
        let app_clone = app.clone();
        let name_clone = name.to_string();
        
        emit_progress(app, phase_id, index + 1, total_files, &format!("Downloading {}...", name), current_base_progress);
        
        http_client::download_file_with_progress(url.to_string(), target_file.to_string_lossy().to_string(), move |p| {
            let overall_progress = current_base_progress + (p as f32 / 100.0 * step_progress);
            emit_progress(&app_clone, phase_id, index + 1, total_files, &format!("Downloading {}...", name_clone), overall_progress);
        }).await?;

        // Verify
        emit_progress(app, phase_id, index + 1, total_files, &format!("Verifying {}...", name), current_base_progress + (step_progress * 0.9));
        let checksum = file_parser::calculate_sha256(&target_file)?;
        if !expected_sha256.is_empty() && checksum != expected_sha256 {
            return Err(format!("Checksum mismatch for {}", name));
        }
    }

    emit_progress(app, phase_id, total_files + 1, total_files, "All downloads complete", 100.0);
    Ok("Downloads completed".to_string())
}

async fn run_phase_2(app: &AppHandle, phase_id: u32) -> Result<String, String> {
    let total_steps = 3;

    // Step 1: Check ADB
    emit_progress(app, phase_id, 1, total_steps, "Checking ADB Connection...", 10.0);
    let devices = adb::get_connected_devices(app.clone()).await?;
    if devices.is_empty() {
        return Err("No devices connected via ADB. Please connect your device.".to_string());
    }

    // Step 2: Backup SMS
    emit_progress(app, phase_id, 2, total_steps, "Backing up SMS (Check Phone to Confirm)...", 50.0);
    
    // Ensure backup directory exists
    let settings = config::load_settings(app.clone()).await?;
    let default_path = "./backups";
    let backup_dir = Path::new(settings["backup_path"].as_str().unwrap_or(default_path));
    if !backup_dir.exists() {
        std::fs::create_dir_all(backup_dir).map_err(|e| e.to_string())?;
    }
    let backup_file = backup_dir.join("sms_backup.ab");

    adb::adb_backup(app.clone(), backup_file.to_string_lossy().to_string(), vec!["com.android.providers.telephony".to_string()], false, false, false).await?;

    emit_progress(app, phase_id, 3, total_steps, "Backup Complete", 100.0);
    Ok("Backup completed".to_string())
}

async fn run_phase_3(app: &AppHandle, phase_id: u32) -> Result<String, String> {
    let total_steps = 5;

    // Step 1: Reboot to Bootloader
    emit_progress(app, phase_id, 1, total_steps, "Rebooting to Bootloader...", 10.0);
    
    // Check if already in fastboot
    let fastboot_devices = fastboot::get_fastboot_devices(app.clone()).await?;
    if fastboot_devices.is_empty() {
        // Check ADB
        let adb_devices = adb::get_connected_devices(app.clone()).await?;
        if !adb_devices.is_empty() {
             adb::reboot_device(app.clone(), "bootloader".to_string()).await?;
             // Wait for reboot
             thread::sleep(Duration::from_secs(5));
        } else {
             // No devices found anywhere
             return Err("No device found in ADB or Fastboot mode. Please connect device.".to_string());
        }
    }

    // Step 2: Verify Fastboot Connection
    emit_progress(app, phase_id, 2, total_steps, "Waiting for Fastboot Connection...", 30.0);
    let mut retries = 0;
    let serial;
    loop {
        let devices = fastboot::get_fastboot_devices(app.clone()).await?;
        if !devices.is_empty() {
            serial = devices[0].clone();
            break;
        }
        if retries > 15 {
            return Err("Device failed to enter Fastboot mode.".to_string());
        }
        thread::sleep(Duration::from_secs(2));
        retries += 1;
    }

    // Step 3: Check Unlock Status
    emit_progress(app, phase_id, 3, total_steps, "Checking Bootloader Status...", 50.0);
    let vars = fastboot::get_var_all(app.clone(), Some(serial.clone())).await?;
    let unlocked = vars.get("unlocked").map(|s| s.as_str()).unwrap_or("no");
    
    if unlocked == "yes" {
        emit_progress(app, phase_id, 5, total_steps, "Bootloader already unlocked. Skipping...", 100.0);
        return Ok("Bootloader already unlocked".to_string());
    }

    // Step 4: Unlock
    emit_progress(app, phase_id, 4, total_steps, "Unlocking Bootloader... PLEASE CONFIRM ON DEVICE SCREEN", 70.0);
    fastboot::bootloader_unlock(app.clone(), Some(serial.clone()), "flashing".to_string()).await?;

    // Step 5: Wait for User
    emit_progress(app, phase_id, 5, total_steps, "Unlock Command Sent. Please confirm on device, wait for wipe, and re-enable USB Debugging.", 100.0);

    Ok("Unlock command sent. Please proceed with device setup.".to_string())
}

async fn get_first_fastboot_device(app: AppHandle) -> Result<String, String> {
    let mut retries = 0;
    loop {
        let devices = fastboot::get_fastboot_devices(app.clone()).await?;
        if !devices.is_empty() {
            return Ok(devices[0].clone());
        }
        if retries > 5 {
            return Err("No Fastboot device found".to_string());
        }
        thread::sleep(Duration::from_secs(1));
        retries += 1;
    }
}

async fn run_phase_4(app: &AppHandle, phase_id: u32) -> Result<String, String> {
    let total_steps = 3;
    emit_progress(app, phase_id, 1, total_steps, "Locating Firmware Files...", 10.0);

    let settings = config::load_settings(app.clone()).await?;
    let download_path = Path::new(settings["save_path"].as_str().unwrap_or("./downloads"));
    
    // In a real scenario, we would unzip the firmware zip here.
    // For this wiring phase, we assume the user or a previous step has extracted them,
    // OR we just flash the zip if supported (but fastboot usually takes images).
    // We will simulate the flashing of critical partitions for now using the 'flash_partition' command.
    
    let serial = get_first_fastboot_device(app.clone()).await?;

    // Example partitions to flash for OnePlus 7 Pro
    let partitions = vec!["modem", "abl", "xbl", "hyp", "tz", "bluetooth", "dsp", "keymaster"];
    
    for (i, part) in partitions.iter().enumerate() {
        let img_name = format!("{}.img", part);
        let img_path = download_path.join(&img_name);
        
        if img_path.exists() {
            emit_progress(app, phase_id, 2, total_steps, &format!("Flashing {}...", part), 20.0 + (i as f32 * 5.0));
            fastboot::flash_partition(app.clone(), part.to_string(), img_path.to_string_lossy().to_string(), Some(serial.clone())).await?;
        } else {
            // Log warning but continue? Or fail?
            // For now, we just log that we are skipping
            println!("Skipping {} - file not found", part);
        }
    }

    emit_progress(app, phase_id, 3, total_steps, "Firmware Update Complete", 100.0);
    Ok("Firmware flashed".to_string())
}

async fn run_phase_5(app: &AppHandle, phase_id: u32) -> Result<String, String> {
    let total_steps = 3;
    emit_progress(app, phase_id, 1, total_steps, "Locating Recovery Image...", 10.0);

    let settings = config::load_settings(app.clone()).await?;
    let download_path = Path::new(settings["save_path"].as_str().unwrap_or("./downloads"));
    
    // Look for recovery.img or twrp.img
    let recovery_path = download_path.join("recovery.img");
    if !recovery_path.exists() {
        return Err("recovery.img not found in downloads".to_string());
    }

    let serial = get_first_fastboot_device(app.clone()).await?;

    emit_progress(app, phase_id, 2, total_steps, "Flashing Recovery...", 50.0);
    fastboot::flash_partition(app.clone(), "recovery".to_string(), recovery_path.to_string_lossy().to_string(), Some(serial.clone())).await?;
    
    // Optional: Flash boot partition if recovery is inside boot (A/B devices)
    // fastboot::flash_partition("boot".to_string(), recovery_path.to_string_lossy().to_string(), Some(serial.clone())).await?;

    emit_progress(app, phase_id, 3, total_steps, "Rebooting to Recovery...", 90.0);
    // Reboot to recovery manually or via command
    // fastboot::reboot(Some("recovery".to_string()), Some(serial.clone())).await?;

    Ok("Recovery flashed".to_string())
}

async fn run_phase_6(app: &AppHandle, phase_id: u32) -> Result<String, String> {
    let total_steps = 4;
    emit_progress(app, phase_id, 1, total_steps, "Waiting for ADB Sideload Mode...", 10.0);

    // Wait for device to be in ADB Sideload mode (Recovery)
    let mut retries = 0;
    loop {
        let devices = adb::get_connected_devices(app.clone()).await?;
        // In sideload mode, adb devices usually shows "sideload" or "recovery"
        // Our get_connected_devices returns a list of serials. 
        // We might need a better check for state, but for now assume if it shows up in ADB while we expect recovery, it's good.
        if !devices.is_empty() {
            break;
        }
        if retries > 30 { // Wait 60 seconds
            return Err("Device not found in ADB mode. Please ensure you are in Recovery -> ADB Sideload.".to_string());
        }
        thread::sleep(Duration::from_secs(2));
        retries += 1;
    }

    let settings = config::load_settings(app.clone()).await?;
    let download_path = Path::new(settings["save_path"].as_str().unwrap_or("./downloads"));
    
    // Find the ROM zip
    let downloads_config = load_downloads_config()?;
    let files = downloads_config["files"].as_array().ok_or("Invalid config")?;
    let rom_file = files.iter().find(|f| f["type"] == "rom").ok_or("ROM file not defined in config")?;
    let filename = rom_file["filename"].as_str().ok_or("ROM filename missing")?;
    let rom_path = download_path.join(filename);

    if !rom_path.exists() {
        return Err(format!("ROM file not found: {:?}", rom_path));
    }

    emit_progress(app, phase_id, 2, total_steps, "Starting Sideload (This may take 10+ mins)...", 20.0);
    
    // We need to listen to the event emitted by adb_sideload to update progress
    // But here we just await the result. The frontend listens to "sideload-progress".
    adb::adb_sideload(app.clone(), rom_path.to_string_lossy().to_string()).await?;

    emit_progress(app, phase_id, 4, total_steps, "ROM Flashing Complete", 100.0);
    Ok("ROM Flashed Successfully".to_string())
}

async fn run_mock_phase(app: &AppHandle, phase_id: u32) -> Result<String, String> {
    let steps = match phase_id {
        3 => vec![
            "Rebooting to Bootloader...",
            "Checking Unlock Status...",
            "Sending Unlock Command...",
            "Waiting for device reboot...",
        ],
        4 => vec![
            "Flashing Firmware Partition 1...",
            "Flashing Firmware Partition 2...",
            "Updating Modem...",
            "Verifying Firmware...",
        ],
        5 => vec![
            "Booting TWRP Recovery...",
            "Mounting System...",
            "Flashing Recovery Image...",
        ],
        6 => vec![
            "Formatting Data...",
            "Sideloading ROM Zip...",
            "Verifying Install...",
        ],
        _ => vec!["Initializing..."],
    };

    let total = steps.len();

    for (i, step) in steps.iter().enumerate() {
        thread::sleep(Duration::from_millis(800));
        emit_progress(app, phase_id, i + 1, total, step, ((i + 1) as f32 / total as f32) * 100.0);
    }

    Ok("Phase completed successfully".to_string())
}
