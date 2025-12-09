use tauri::{command, AppHandle};
use sha2::{Sha256, Digest};
use md5;
use std::fs::{self, File};
use std::io::{Read, Write, BufReader};
use crate::commands::adb;
use crate::commands::module_manager;
use crate::commands::http_client;
use serde::{Deserialize, Serialize};
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce // Or `Aes128Gcm`
};
use rand::RngCore;

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionEntry {
    pub package: String,
    pub permission: String,
    pub status: String,
}

#[command]
pub async fn audit_permissions(app: AppHandle) -> Result<Vec<PermissionEntry>, String> {
    // We will check for dangerous permissions: Camera, Location, Mic, Contacts, SMS
    // Command: adb shell appops query-op CAMERA allow
    // Better approach: List all packages and check their requested permissions is too slow.
    // Fast approach: Check ops for specific dangerous ops.
    
    let ops = vec![
        ("CAMERA", "android:camera"),
        ("FINE_LOCATION", "android:fine_location"),
        ("RECORD_AUDIO", "android:record_audio"),
        ("READ_SMS", "android:read_sms"),
        ("READ_CONTACTS", "android:read_contacts"),
    ];
    
    let mut entries = Vec::new();
    
    for (perm_name, op_code) in ops {
        // "appops query-op <OP> allow" returns "PackageName"
        // Output format: "Package: com.foo.bar"
        let output = adb::run_adb_shell(app.clone(), format!("appops query-op {} allow", op_code)).await?;
        
        for line in output.lines() {
            // Example output: "com.android.camera2" (It might just list package names directly on some versions)
            // Or "Package: com.android.camera2"
            let pkg = line.replace("Package: ", "").trim().to_string();
            if !pkg.is_empty() {
                entries.push(PermissionEntry {
                    package: pkg,
                    permission: perm_name.to_string(),
                    status: "Granted".to_string(),
                });
            }
        }
    }
    
    Ok(entries)
}

#[command]
pub async fn revoke_permission(app: AppHandle, package: String, permission: String) -> Result<String, String> {
    // Map friendly name back to android permission
    let android_perm = match permission.as_str() {
        "CAMERA" => "android.permission.CAMERA",
        "FINE_LOCATION" => "android.permission.ACCESS_FINE_LOCATION",
        "RECORD_AUDIO" => "android.permission.RECORD_AUDIO",
        "READ_SMS" => "android.permission.READ_SMS",
        "READ_CONTACTS" => "android.permission.READ_CONTACTS",
        _ => return Err("Unknown permission type".to_string()),
    };
    
    adb::run_adb_shell(app, format!("pm revoke {} {}", package, android_perm)).await
}

#[command]
pub async fn encrypt_file(input_path: String, output_path: String, password: String) -> Result<String, String> {
    // Key derivation (Simple SHA256 of password for now - in prod use Argon2)
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let key_bytes = hasher.finalize();
    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    // Generate random nonce
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Read file
    let plaintext = fs::read(&input_path).map_err(|e| e.to_string())?;

    // Encrypt
    let ciphertext = cipher.encrypt(nonce, plaintext.as_ref())
        .map_err(|e| format!("Encryption failed: {}", e))?;

    // Write Nonce + Ciphertext
    let mut output_file = File::create(&output_path).map_err(|e| e.to_string())?;
    output_file.write_all(&nonce_bytes).map_err(|e| e.to_string())?;
    output_file.write_all(&ciphertext).map_err(|e| e.to_string())?;

    Ok(output_path)
}

#[command]
pub async fn decrypt_file(input_path: String, output_path: String, password: String) -> Result<String, String> {
    // Key derivation
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let key_bytes = hasher.finalize();
    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    // Read file
    let mut file = File::open(&input_path).map_err(|e| e.to_string())?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;

    if buffer.len() < 12 {
        return Err("File too short".to_string());
    }

    // Split Nonce and Ciphertext
    let (nonce_bytes, ciphertext) = buffer.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    // Decrypt
    let plaintext = cipher.decrypt(nonce, ciphertext)
        .map_err(|e| format!("Decryption failed (Wrong password?): {}", e))?;

    fs::write(&output_path, plaintext).map_err(|e| e.to_string())?;

    Ok(output_path)
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
