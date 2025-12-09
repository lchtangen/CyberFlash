use tauri::command;
use std::process::Command;

#[command]
pub async fn trigger_rescue_mode(strategy: String) -> Result<String, String> {
    println!("Executing Rescue Strategy: {}", strategy);
    
    match strategy.as_str() {
        "reboot_recovery" => {
            // Try ADB first
            // We ignore errors because the device might be in a state where ADB doesn't work, 
            // but we want to try Fastboot next anyway.
            let _ = Command::new("adb").args(["reboot", "recovery"]).output();
            
            // Try Fastboot (some devices support 'fastboot reboot recovery')
            // If not, at least 'fastboot reboot' might break the loop if keys are held
            let _ = Command::new("fastboot").args(["reboot", "recovery"]).output();
            
            Ok("Rescue signal sent: Reboot to Recovery".to_string())
        },
        "reboot_bootloader" => {
             let _ = Command::new("adb").args(["reboot", "bootloader"]).output();
             let _ = Command::new("fastboot").args(["reboot-bootloader"]).output();
             Ok("Rescue signal sent: Reboot to Bootloader".to_string())
        },
        "force_restart" => {
            let _ = Command::new("adb").args(["reboot"]).output();
            let _ = Command::new("fastboot").args(["reboot"]).output();
            Ok("Rescue signal sent: Force Restart".to_string())
        },
        _ => Err("Unknown rescue strategy".to_string())
    }
}
