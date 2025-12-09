use tauri::command;
use crate::commands::flash_as_code::{FlashConfig, FlashStep};
use std::path::Path;

#[command]
pub fn generate_op7pro_flash_config(rom_type: String, path: String) -> FlashConfig {
    let mut steps = Vec::new();

    if rom_type == "stock_fastboot" {
        // Stock Fastboot ROM (extracted images)
        // Assuming path is a folder path containing images
        
        // Critical partitions for OnePlus 7 Pro (Guacamole)
        let mut partitions = vec![
            "boot", "dtbo", "vbmeta", "modem", "bluetooth", "dsp", "logo", "abl", "aop", "cmnlib", "cmnlib64", "devcfg", "qupfw", "storsec", "tz", "hyp", "keymaster", "xbl", "xbl_config"
        ];
        
        // Check for init_boot (Android 13/14+)
        if Path::new(&path).join("init_boot.img").exists() {
            partitions.push("init_boot");
        }
        
        // System/Vendor/Product/ODM are large, handle separately
        let large_partitions = vec!["system", "vendor", "product", "odm"];

        steps.push(FlashStep::Reboot { mode: "bootloader".into() });
        
        // Flash Firmware & Boot components to both slots
        for part in partitions {
            steps.push(FlashStep::FlashImage { 
                partition: part.to_string(), 
                file: format!("{}/{}.img", path, part),
                slot: Some("all".into()) // fastboot flash partition --slot=all
            });
        }

        // Flash Large Partitions
        for part in large_partitions {
             steps.push(FlashStep::FlashImage { 
                partition: part.to_string(), 
                file: format!("{}/{}.img", path, part),
                slot: Some("all".into())
            });
        }
        
        steps.push(FlashStep::Wipe { partitions: vec!["userdata".into()] });
        steps.push(FlashStep::Reboot { mode: "system".into() });

    } else if rom_type == "custom_recovery" {
        // Custom ROM via Recovery (Sideload)
        // 1. Boot TWRP/Recovery
        // 2. Sideload Zip
        // 3. Format Data
        
        steps.push(FlashStep::Reboot { mode: "recovery".into() });
        steps.push(FlashStep::Wait { seconds: 15 }); // Wait for recovery to initialize
        
        // Sideload the ROM
        steps.push(FlashStep::Sideload { file: path });
        
        // Format Data (often needed for encryption changes)
        steps.push(FlashStep::Wipe { partitions: vec!["userdata".into()] }); 
        
        steps.push(FlashStep::Reboot { mode: "system".into() });
    }

    FlashConfig {
        name: "OnePlus 7 Pro Flash Sequence".into(),
        device: "guacamole".into(),
        version: "1.0".into(),
        steps,
    }
}
