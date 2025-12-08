use tauri::{command, AppHandle};
use tauri_plugin_shell::ShellExt;
use std::collections::HashMap;

#[command]
pub async fn get_fastboot_devices(app: AppHandle) -> Result<Vec<String>, String> {
    let output = app.shell().sidecar("fastboot")
        .map_err(|e| e.to_string())?
        .args(["devices"])
        .output()
        .await
        .map_err(|e| e.to_string())?;
        
    let output_str = String::from_utf8_lossy(&output.stdout);
    let devices: Vec<String> = output_str
        .lines()
        .map(|line| line.split_whitespace().next().unwrap_or("").to_string())
        .filter(|s| !s.is_empty())
        .collect();
        
    Ok(devices)
}

#[command]
pub async fn get_var_all(app: AppHandle, serial: Option<String>) -> Result<HashMap<String, String>, String> {
    let mut cmd = app.shell().sidecar("fastboot").map_err(|e| e.to_string())?;
    if let Some(s) = serial {
        cmd = cmd.args(["-s", &s]);
    }
    cmd = cmd.args(["getvar", "all"]);

    let output = cmd.output().await.map_err(|e| e.to_string())?;
    
    // Fastboot getvar all outputs to stderr usually
    let output_str = String::from_utf8_lossy(&output.stderr);
    let mut vars = HashMap::new();

    for line in output_str.lines() {
        // Format: (bootloader) key: value
        if let Some(idx) = line.find(':') {
            let key = line[..idx].replace("(bootloader) ", "").trim().to_string();
            let value = line[idx+1..].trim().to_string();
            vars.insert(key, value);
        }
    }

    Ok(vars)
}

#[command]
pub async fn set_active_slot(app: AppHandle, slot: String, serial: Option<String>) -> Result<String, String> {
    let mut cmd = app.shell().sidecar("fastboot").map_err(|e| e.to_string())?;
    if let Some(s) = serial {
        cmd = cmd.args(["-s", &s]);
    }
    cmd = cmd.args(["set_active", &slot]);

    let output = cmd.output().await.map_err(|e| e.to_string())?;
    
    if output.status.success() {
        Ok(format!("Switched to slot {}", slot))
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[command]
pub async fn bootloader_unlock(app: AppHandle, serial: Option<String>, method: String) -> Result<String, String> {
    let cmd_str = if method == "flashing" { "flashing" } else { "oem" };
    let arg_str = "unlock";

    let mut cmd = app.shell().sidecar("fastboot").map_err(|e| e.to_string())?;
    if let Some(s) = serial {
        cmd = cmd.args(["-s", &s]);
    }
    cmd = cmd.args([cmd_str, arg_str]);

    let output = cmd.output().await.map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok("Unlock command sent. Check device screen.".to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[command]
pub async fn bootloader_lock(app: AppHandle, serial: Option<String>) -> Result<String, String> {
    let mut cmd = app.shell().sidecar("fastboot").map_err(|e| e.to_string())?;
    if let Some(s) = serial {
        cmd = cmd.args(["-s", &s]);
    }
    cmd = cmd.args(["flashing", "lock"]);

    let output = cmd.output().await.map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok("Lock command sent. Check device screen.".to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[command]
pub async fn check_bootloader_unlocked(app: AppHandle, serial: Option<String>) -> Result<bool, String> {
    let vars = get_var_all(app, serial).await?;
    if let Some(unlocked) = vars.get("unlocked") {
        return Ok(unlocked == "yes");
    }
    Err("Could not determine unlock status".to_string())
}

#[command]
pub async fn detect_ab_slots(app: AppHandle, serial: Option<String>) -> Result<bool, String> {
    let vars = get_var_all(app, serial).await?;
    if let Some(slot_count) = vars.get("slot-count") {
        return Ok(slot_count == "2");
    }
    // Fallback check for current-slot
    if vars.contains_key("current-slot") {
        return Ok(true);
    }
    Ok(false)
}

#[command]
pub async fn flash_partition(app: AppHandle, partition: String, path: String, serial: Option<String>) -> Result<String, String> {
    let mut cmd = app.shell().sidecar("fastboot").map_err(|e| e.to_string())?;
    if let Some(s) = serial {
        cmd = cmd.args(["-s", &s]);
    }
    cmd = cmd.args(["flash", &partition, &path]);

    let output = cmd.output().await.map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(format!("Flashed {} successfully", partition))
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[command]
pub async fn erase_partition(app: AppHandle, partition: String, serial: Option<String>) -> Result<String, String> {
    let mut cmd = app.shell().sidecar("fastboot").map_err(|e| e.to_string())?;
    if let Some(s) = serial {
        cmd = cmd.args(["-s", &s]);
    }
    cmd = cmd.args(["erase", &partition]);

    let output = cmd.output().await.map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(format!("Erased {} successfully", partition))
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[command]
pub async fn disable_verity(app: AppHandle, vbmeta_path: String, serial: Option<String>) -> Result<String, String> {
    let mut cmd = app.shell().sidecar("fastboot").map_err(|e| e.to_string())?;
    if let Some(s) = serial {
        cmd = cmd.args(["-s", &s]);
    }
    // fastboot --disable-verity --disable-verification flash vbmeta vbmeta.img
    cmd = cmd.args(["--disable-verity", "--disable-verification", "flash", "vbmeta", &vbmeta_path]);

    let output = cmd.output().await.map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok("Verity disabled successfully".to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[command]
pub async fn format_data(app: AppHandle, serial: Option<String>) -> Result<String, String> {
    let mut cmd = app.shell().sidecar("fastboot").map_err(|e| e.to_string())?;
    if let Some(s) = serial {
        cmd = cmd.args(["-s", &s]);
    }
    cmd = cmd.args(["-w"]); // -w wipes userdata and cache

    let output = cmd.output().await.map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok("Data formatted successfully".to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[command]
pub async fn wait_for_bootloader(app: AppHandle) -> Result<String, String> {
    let output = app.shell().sidecar("fastboot")
        .map_err(|e| e.to_string())?
        .args(["wait-for-bootloader"])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok("Device detected in bootloader".to_string())
    } else {
        Err("Wait timed out or failed".to_string())
    }
}

#[command]
pub async fn fastboot_boot(app: AppHandle, image_path: String, serial: Option<String>) -> Result<String, String> {
    let mut cmd = app.shell().sidecar("fastboot").map_err(|e| e.to_string())?;
    if let Some(s) = serial {
        cmd = cmd.args(["-s", &s]);
    }
    cmd = cmd.args(["boot", &image_path]);

    let output = cmd.output().await.map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok("Booting image...".to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[command]
pub async fn get_partition_info(app: AppHandle, partition: String, serial: Option<String>) -> Result<String, String> {
    let vars = get_var_all(app, serial).await?;
    // Look for partition-size:partition or partition-type:partition
    let size_key = format!("partition-size:{}", partition);
    let type_key = format!("partition-type:{}", partition);
    
    let size = vars.get(&size_key).cloned().unwrap_or("unknown".to_string());
    let p_type = vars.get(&type_key).cloned().unwrap_or("unknown".to_string());
    
    Ok(format!("Size: {}, Type: {}", size, p_type))
}

#[command]
pub async fn check_dynamic_partitions(app: AppHandle, serial: Option<String>) -> Result<bool, String> {
    let vars = get_var_all(app, serial).await?;
    // Check for 'super' partition existence or is-logical
    if vars.contains_key("partition-size:super") {
        return Ok(true);
    }
    Ok(false)
}

#[command]
pub async fn flash_all_partitions(app: AppHandle, folder_path: String, serial: Option<String>) -> Result<String, String> {
    // This is a simplified "flash-all" logic
    // In reality, we'd parse a list or look for specific files
    let partitions = vec!["boot", "dtbo", "vbmeta", "recovery", "system", "vendor", "product"];
    let mut log = String::new();

    for part in partitions {
        let file_path = std::path::Path::new(&folder_path).join(format!("{}.img", part));
        if file_path.exists() {
            match flash_partition(app.clone(), part.to_string(), file_path.to_string_lossy().to_string(), serial.clone()).await {
                Ok(_) => log.push_str(&format!("Flashed {}\n", part)),
                Err(e) => log.push_str(&format!("Failed to flash {}: {}\n", part, e)),
            }
        }
    }
    Ok(log)
}

#[command]
pub async fn set_active_slot_and_reboot(app: AppHandle, slot: String, serial: Option<String>) -> Result<String, String> {
    set_active_slot(app.clone(), slot.clone(), serial.clone()).await?;
    reboot(app, None, serial).await?;
    Ok(format!("Switched to slot {} and rebooting", slot))
}






#[command]
#[allow(dead_code)]
pub async fn reboot(app: AppHandle, target: Option<String>, serial: Option<String>) -> Result<String, String> {
    let mut cmd = app.shell().sidecar("fastboot").map_err(|e| e.to_string())?;
    if let Some(s) = serial {
        cmd = cmd.args(["-s", &s]);
    }
    cmd = cmd.args(["reboot"]);
    if let Some(t) = target {
        cmd = cmd.args([&t]);
    }

    let output = cmd.output().await.map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok("Rebooting...".to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
