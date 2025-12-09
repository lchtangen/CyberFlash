use tauri::{command, AppHandle};
use crate::commands::adb;

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
