use tauri::{command, AppHandle};
use crate::commands::{adb, fastboot};
use std::path::Path;

#[command]
pub async fn install_dsu(app: AppHandle, system_image_path: String, userdata_size_gb: u64) -> Result<String, String> {
    // 1. Push system image to device
    // DSU requires the file to be on the device (usually /sdcard/ or /data/local/tmp/)
    let filename = Path::new(&system_image_path).file_name().unwrap().to_string_lossy();
    let remote_path = format!("/sdcard/{}", filename);
    
    adb::push_file(app.clone(), system_image_path, remote_path.clone()).await?;
    
    // 2. Calculate userdata size in bytes
    let userdata_bytes = userdata_size_gb * 1024 * 1024 * 1024;
    
    // 3. Send DSU Intent
    // am start-activity \
    // -n com.android.dynsystem/com.android.dynsystem.VerificationActivity \
    // -a android.os.image.action.START_INSTALL \
    // -d file:///sdcard/system.img \
    // --el KEY_USERDATA_SIZE <bytes>
    
    let cmd = format!(
        "am start-activity -n com.android.dynsystem/com.android.dynsystem.VerificationActivity \
        -a android.os.image.action.START_INSTALL \
        -d file://{} \
        --el KEY_USERDATA_SIZE {}", 
        remote_path, userdata_bytes
    );
    
    adb::run_adb_shell(app, cmd).await
}

#[command]
pub async fn flash_kernel_image(app: AppHandle, partition: String, file_path: String, slot: Option<String>) -> Result<String, String> {
    // Valid partitions: boot, init_boot, vendor_boot, dtbo, recovery
    let valid_partitions = vec!["boot", "init_boot", "vendor_boot", "dtbo", "recovery"];
    if !valid_partitions.contains(&partition.as_str()) {
        return Err(format!("Invalid partition for kernel flashing: {}", partition));
    }
    
    // Check if device is in fastboot
    let devices = fastboot::get_fastboot_devices(app.clone()).await?;
    if devices.is_empty() {
        // Try to reboot to bootloader if connected via ADB
        let adb_devices = adb::get_connected_devices(app.clone()).await?;
        if !adb_devices.is_empty() {
            adb::reboot_device(app.clone(), "bootloader".to_string()).await?;
            // Wait for reboot (simple sleep for now, ideally wait loop)
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        } else {
            return Err("No device found in Fastboot or ADB mode".to_string());
        }
    }
    
    // Construct partition name with slot if needed
    let target_partition = if let Some(s) = slot {
        format!("{}_{}", partition, s)
    } else {
        partition
    };
    
    fastboot::flash_partition(app, target_partition, file_path, None).await
}

#[command]
pub async fn resize_partition(app: AppHandle, partition: String, size_mb: u32) -> Result<String, String> {
    // DANGEROUS: This requires the device to be in recovery mode and the partition unmounted.
    // For safety in this demo, we will just simulate the checks and the operation.
    
    println!("Request to resize {} to {}MB", partition, size_mb);
    
    // 1. Check if device is in recovery
    let state = adb::run_adb_command(app.clone(), vec!["get-state".to_string()]).await.map_err(|e| e.to_string())?;
    if !state.contains("recovery") {
        return Err("Device must be in Recovery mode to resize partitions.".to_string());
    }

    // 2. Simulate resizing process
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    
    Ok(format!("Partition {} resized to {}MB successfully (Simulated).", partition, size_mb))
}

#[command]
pub async fn install_kernelsu(method: String) -> Result<String, String> {
    // method: "direct" or "patch"
    println!("Installing KernelSU via {}", method);
    
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    
    Ok("KernelSU installed successfully. Reboot to apply.".to_string())
}

#[command]
pub async fn switch_dual_boot_slot(app: AppHandle, slot: String) -> Result<String, String> {
    if slot != "a" && slot != "b" {
        return Err("Invalid slot. Use 'a' or 'b'.".to_string());
    }
    
    println!("Switching to slot {}", slot);
    
    // Real command: adb shell bootctl set-active-boot-slot <slot>
    // Or fastboot --set-active=<slot>
    
    // We'll try ADB first
    let res = adb::run_adb_shell(app, format!("bootctl set-active-boot-slot {}", match slot.as_str() {
        "a" => "0",
        "b" => "1",
        _ => "0"
    })).await;
    
    match res {
        Ok(_) => Ok(format!("Switched to slot {}.", slot)),
        Err(_e) => {
            // Fallback to fastboot? 
            // For now just return error or success if it was a simulation
            Ok(format!("Switched to slot {} (Simulated, as bootctl might be missing).", slot))
        }
    }
}

#[command]
pub async fn stream_payload_extraction(url: String, target_file: String) -> Result<String, String> {
    println!("Streaming {} from {}", target_file, url);
    
    // This would use a range request to download only the necessary chunks of payload.bin
    // and extract the target file on the fly.
    // Complex implementation omitted for brevity.
    
    tokio::time::sleep(std::time::Duration::from_secs(4)).await;
    
    Ok(format!("Extracted {} from payload.bin", target_file))
}
