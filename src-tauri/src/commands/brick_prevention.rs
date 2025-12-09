use tauri::command;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BrickRisk {
    pub level: String, // "Safe", "Warning", "Critical"
    pub message: String,
    pub details: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FlashContext {
    pub device_model: String,
    pub current_firmware: String,
    pub target_rom: String,
    pub target_android_version: String,
}

#[command]
pub fn check_brick_risk(context: FlashContext) -> BrickRisk {
    let device = context.device_model.to_lowercase();
    let firmware = context.current_firmware.to_lowercase();
    let rom = context.target_rom.to_lowercase();
    let android_ver = context.target_android_version.to_lowercase();

    // Rule 1: OnePlus 7 Pro Android 14 (Lineage 21) requires OOS 12 Firmware
    if device.contains("oneplus 7 pro") || device.contains("guacamole") {
        if (rom.contains("lineage") && rom.contains("21")) || android_ver.contains("14") {
            if firmware.contains("11") || firmware.contains("10") {
                return BrickRisk {
                    level: "Critical".to_string(),
                    message: "Firmware Mismatch Detected".to_string(),
                    details: Some("Android 14 ROMs require OxygenOS 12 Firmware (H.41 or later) on BOTH slots before flashing. Flashing now will cause a crash to Qualcomm CrashDump Mode.".to_string()),
                };
            }
        }
    }

    // Rule 2: Pixel 6 Anti-Rollback
    if device.contains("pixel 6") {
        if android_ver.contains("12") && firmware.contains("13") {
            return BrickRisk {
                level: "Critical".to_string(),
                message: "Anti-Rollback Trigger Warning".to_string(),
                details: Some("Downgrading from Android 13 to 12 on Pixel 6 will trigger hardware anti-rollback and HARD BRICK the device. Do not proceed.".to_string()),
            };
        }
    }

    // Rule 3: Xiaomi Anti-Rollback (General)
    if device.contains("xiaomi") || device.contains("redmi") {
        if firmware.contains("arb:4") && android_ver.contains("old") {
             return BrickRisk {
                level: "Warning".to_string(),
                message: "Xiaomi Anti-Rollback Check".to_string(),
                details: Some("Ensure you are not flashing a ROM with an older security patch level than your current firmware.".to_string()),
            };
        }
    }

    // Default: Safe
    BrickRisk {
        level: "Safe".to_string(),
        message: "No known brick risks detected.".to_string(),
        details: None,
    }
}
