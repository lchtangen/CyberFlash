use tauri::{command, AppHandle};
use sha2::{Sha256, Digest};
use md5;
use std::fs::File;
use std::io::{Read, BufReader};
use crate::commands::adb;
use crate::commands::module_manager;
use crate::commands::http_client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionEntry {
    pub package: String,
    pub permission: String,
    pub status: String,
}

#[command]
pub fn verify_file_hash(file_path: String, expected_hash: String, algorithm: String) -> Result<bool, String> {
    let file = File::open(&file_path).map_err(|e| e.to_string())?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).map_err(|e| e.to_string())?;

    let calculated_hash = match algorithm.to_lowercase().as_str() {
        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.update(&buffer);
            format!("{:x}", hasher.finalize())
        },
        "md5" => {
            let digest = md5::compute(&buffer);
            format!("{:x}", digest)
        },
        _ => return Err("Unsupported algorithm".into())
    };

    Ok(calculated_hash.eq_ignore_ascii_case(&expected_hash))
}

#[command]
pub fn calculate_file_hash(file_path: String, algorithm: String) -> Result<String, String> {
    let file = File::open(&file_path).map_err(|e| e.to_string())?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).map_err(|e| e.to_string())?;

    match algorithm.to_lowercase().as_str() {
        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.update(&buffer);
            Ok(format!("{:x}", hasher.finalize()))
        },
        "md5" => {
            let digest = md5::compute(&buffer);
            Ok(format!("{:x}", digest))
        },
        _ => Err("Unsupported algorithm".into())
    }
}

#[command]
pub async fn backup_efs_partitions(app: AppHandle) -> Result<String, String> {
    // Critical partitions for Qualcomm devices
    let partitions = vec!["modemst1", "modemst2", "fsg", "fsc"];
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let backup_dir = format!("/sdcard/CyberFlash/Backups/EFS_{}", timestamp);
    
    adb::run_adb_shell(app.clone(), format!("mkdir -p {}", backup_dir)).await?;
    
    for part in partitions {
        // Check if partition exists first
        let check = adb::run_adb_shell(app.clone(), format!("ls /dev/block/bootdevice/by-name/{}", part)).await;
        if check.is_ok() {
            // dd if=/dev/block/bootdevice/by-name/part of=/sdcard/...
            let cmd = format!("dd if=/dev/block/bootdevice/by-name/{} of={}/{}.img", part, backup_dir, part);
            adb::run_adb_shell(app.clone(), cmd).await?;
        }
    }
    
    // Pull to PC
    // In real app, we would pull to local storage
    // For now, we just return the remote path
    Ok(backup_dir)
}

#[command]
pub async fn install_safetynet_fix(app: AppHandle) -> Result<String, String> {
    // URL for Universal SafetyNet Fix (Displax Mod) - Hardcoded for prototype
    // In production, fetch from GitHub API
    let url = "https://github.com/Displax/safetynet-fix/releases/download/v2.4.0-MOD_2.0/safetynet-fix-v2.4.0-MOD_2.0.zip";
    let save_path = "/tmp/safetynet-fix.zip"; // On PC
    
    // Download
    http_client::download_file(app.clone(), url.into(), save_path.into()).await?;
    
    // Install
    module_manager::install_module_zip(app, save_path.into()).await
}

#[command]
pub async fn audit_permissions(app: AppHandle) -> Result<Vec<PermissionEntry>, String> {
    // Check for Location, Camera, Microphone
    let perms = vec![
        "android.permission.ACCESS_FINE_LOCATION",
        "android.permission.CAMERA",
        "android.permission.RECORD_AUDIO"
    ];
    
    let mut results = Vec::new();
    
    for perm in perms {
        // dumpsys package p | grep permission
        // Better: cmd appops query-op <OP> allow
        // OP names: FINE_LOCATION, CAMERA, RECORD_AUDIO
        
        let op = match perm {
            "android.permission.ACCESS_FINE_LOCATION" => "FINE_LOCATION",
            "android.permission.CAMERA" => "CAMERA",
            "android.permission.RECORD_AUDIO" => "RECORD_AUDIO",
            _ => continue
        };
        
        let output = adb::run_adb_shell(app.clone(), format!("cmd appops query-op {} allow", op)).await?;
        // Output format: "PackageName" (sometimes with uid)
        
        for line in output.lines() {
            if !line.trim().is_empty() {
                // Line might be "com.google.android.gms"
                let pkg = line.trim().split_whitespace().next().unwrap_or("").to_string();
                if !pkg.is_empty() {
                    results.push(PermissionEntry {
                        package: pkg,
                        permission: op.to_string(),
                        status: "Allowed".to_string()
                    });
                }
            }
        }
    }
    
    Ok(results)
}

#[command]
pub async fn revoke_permission(app: AppHandle, package: String, permission: String) -> Result<String, String> {
    // permission is the OP name (FINE_LOCATION), map back to android permission
    let android_perm = match permission.as_str() {
        "FINE_LOCATION" => "android.permission.ACCESS_FINE_LOCATION",
        "CAMERA" => "android.permission.CAMERA",
        "RECORD_AUDIO" => "android.permission.RECORD_AUDIO",
        _ => return Err("Unknown permission op".into())
    };
    
    adb::run_adb_shell(app, format!("pm revoke {} {}", package, android_perm)).await
}

#[command]
pub async fn lock_bootloader(app: AppHandle) -> Result<String, String> {
    // WARNING: This wipes data
    // Try standard commands
    // fastboot flashing lock
    // fastboot oem lock
    
    // We need to be in fastboot mode
    // Let's assume we are or try to reboot to it
    
    // For safety, we just return the command to run or run it if user confirmed
    // In this context, the frontend handles confirmation.
    
    // Try 'flashing lock' first (Pixel/Nexus/Modern)
    let res = crate::commands::fastboot::run_fastboot_cmd(app.clone(), vec!["flashing", "lock"]).await;
    if res.is_ok() {
        return res;
    }
    
    // Fallback to 'oem lock' (Older/OnePlus)
    crate::commands::fastboot::run_fastboot_cmd(app, vec!["oem", "lock"]).await
}

#[command]
pub async fn kill_switch(app: AppHandle, revoke_auth: bool) -> Result<String, String> {
    if revoke_auth {
        // Try to remove keys from device if rooted
        let _ = adb::run_adb_shell(app.clone(), "rm /data/misc/adb/adb_keys".into()).await;
    }
    
    // Kill server on PC
    adb::kill_server(app).await
}
