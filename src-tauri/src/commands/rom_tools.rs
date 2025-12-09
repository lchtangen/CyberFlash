use tauri::{command, AppHandle, Manager};
use crate::commands::adb;
use std::fs::File;
use std::io::copy;
use zip::ZipArchive;
use serde_json::Value;

#[command]
pub fn get_device_inventory(app: AppHandle) -> Result<Value, String> {
    // Try to find the config file
    // In dev: ../config/oneplus7pro_inventory.json (relative to src-tauri)
    // In prod: app_config_dir/oneplus7pro_inventory.json
    
    let dev_path = std::path::PathBuf::from("../config/oneplus7pro_inventory.json");
    let prod_path = app.path().app_config_dir().unwrap_or_default().join("oneplus7pro_inventory.json");
    
    let path = if dev_path.exists() {
        dev_path
    } else {
        prod_path
    };

    if !path.exists() {
        return Err(format!("Inventory file not found at {:?}", path));
    }

    let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let json: Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    Ok(json)
}

#[command]
pub async fn extract_file_from_zip(zip_path: String, target_file: String, output_path: String) -> Result<String, String> {
    let file = File::open(&zip_path).map_err(|e| e.to_string())?;
    let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        // Case insensitive match or exact match? Exact for now.
        // Also handle paths inside zip (e.g. "images/boot.img")
        if file.name().ends_with(&target_file) {
            let mut out_file = File::create(&output_path).map_err(|e| e.to_string())?;
            copy(&mut file, &mut out_file).map_err(|e| e.to_string())?;
            return Ok(output_path);
        }
    }
    
    Err(format!("File {} not found in zip", target_file))
}

#[command]
pub fn get_gapps_url(android_version: String, variant: String, arch: String) -> String {
    // NikGApps is popular for Android 10-14
    // OpenGApps is popular for older versions
    // MindTheGapps is standard for LineageOS
    
    let version_int = android_version.parse::<i32>().unwrap_or(14);
    
    if version_int >= 12 {
        // NikGApps (SourceForge)
        // Structure is complex, so we point to the releases page
        format!("https://sourceforge.net/projects/nikgapps/files/Releases/NikGapps-T/")
    } else {
        // OpenGApps
        format!("https://opengapps.org/?arch={}&api={}&variant={}", arch, android_version, variant)
    }
}

#[command]
pub async fn check_firmware_compliance(app: AppHandle, required_version: String) -> Result<bool, String> {
    // Check ro.build.version.release or ro.build.display.id or specific vendor props
    // For OnePlus: ro.build.version.ota
    // For Xiaomi: ro.miui.ui.version.name
    
    let props = adb::run_adb_shell(app, "getprop".into()).await?;
    
    // Simple substring match for now
    if props.contains(&required_version) {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[command]
pub async fn get_device_firmware_info(app: AppHandle) -> Result<String, String> {
    let release = adb::get_prop_value(app.clone(), "ro.build.version.release".into()).await.unwrap_or_default();
    let display = adb::get_prop_value(app.clone(), "ro.build.display.id".into()).await.unwrap_or_default();
    let baseband = adb::get_prop_value(app.clone(), "gsm.version.baseband".into()).await.unwrap_or_default();
    
    Ok(format!("Android: {}\nBuild: {}\nBaseband: {}", release, display, baseband))
}
