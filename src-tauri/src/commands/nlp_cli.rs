use tauri::command;
use serde::Serialize;
use std::process::Command;

#[derive(Serialize)]
pub struct ParsedCommand {
    pub original: String,
    pub commands: Vec<String>,
    pub confidence: f32,
    pub explanation: String,
}

#[command]
pub async fn parse_natural_language(query: String) -> Result<ParsedCommand, String> {
    let query_lower = query.to_lowercase();
    let mut commands = Vec::new();
    let mut explanation = String::new();
    let mut confidence = 0.0;

    // 1. Reboot Logic
    if query_lower.contains("reboot") || query_lower.contains("restart") {
        if query_lower.contains("recovery") || query_lower.contains("twrp") {
            commands.push("adb reboot recovery".to_string());
            explanation = "Rebooting device into Recovery mode (TWRP/Stock).".to_string();
            confidence = 0.95;
        } else if query_lower.contains("bootloader") || query_lower.contains("fastboot") || query_lower.contains("download") {
            commands.push("adb reboot bootloader".to_string());
            explanation = "Rebooting device into Bootloader/Fastboot mode.".to_string();
            confidence = 0.95;
        } else if query_lower.contains("edl") || query_lower.contains("emergency") {
            commands.push("adb reboot edl".to_string());
            explanation = "Attempting to reboot into Qualcomm EDL mode.".to_string();
            confidence = 0.9;
        } else {
            commands.push("adb reboot".to_string());
            explanation = "Performing a normal system reboot.".to_string();
            confidence = 0.85;
        }
    } 
    // 2. Flashing Logic
    else if query_lower.contains("flash") || query_lower.contains("install") {
        if query_lower.contains("recovery") {
             commands.push("fastboot flash recovery recovery.img".to_string());
             explanation = "Queued command to flash recovery partition. Please ensure a recovery image is selected.".to_string();
             confidence = 0.9;
        } else if query_lower.contains("boot") || query_lower.contains("kernel") {
             commands.push("fastboot flash boot boot.img".to_string());
             explanation = "Queued command to flash boot/kernel partition.".to_string();
             confidence = 0.9;
        } else if query_lower.contains("rom") || query_lower.contains("zip") || query_lower.contains("lineage") {
             commands.push("adb sideload rom.zip".to_string());
             explanation = "Assuming ADB Sideload for ROM installation.".to_string();
             confidence = 0.8;
        } else {
             explanation = "I detected a flash intent but need to know which partition (boot, recovery, system) or if it's a ROM zip.".to_string();
             confidence = 0.4;
        }
    } 
    // 3. Sideload Logic
    else if query_lower.contains("sideload") {
        commands.push("adb sideload <filename>".to_string());
        explanation = "Preparing ADB Sideload. You will need to select a file.".to_string();
        confidence = 0.95;
    } 
    // 4. Information/Listing
    else if query_lower.contains("list") || query_lower.contains("show") {
        if query_lower.contains("packages") || query_lower.contains("apps") {
            commands.push("adb shell pm list packages".to_string());
            explanation = "Listing all installed packages on the device.".to_string();
            confidence = 0.95;
        } else if query_lower.contains("devices") {
            commands.push("adb devices".to_string());
            commands.push("fastboot devices".to_string());
            explanation = "Checking for connected ADB and Fastboot devices.".to_string();
            confidence = 0.95;
        } else if query_lower.contains("battery") {
            commands.push("adb shell dumpsys battery".to_string());
            explanation = "Retrieving detailed battery statistics.".to_string();
            confidence = 0.95;
        }
    }
    // 5. Wiping/Reset
    else if query_lower.contains("wipe") || query_lower.contains("reset") || query_lower.contains("format") {
        if query_lower.contains("data") || query_lower.contains("factory") {
            commands.push("fastboot -w".to_string());
            explanation = "WARNING: This will wipe all user data (Factory Reset).".to_string();
            confidence = 0.9;
        } else if query_lower.contains("cache") {
            commands.push("fastboot erase cache".to_string());
            explanation = "Erasing cache partition.".to_string();
            confidence = 0.95;
        }
    }
    // 6. Unlock/Lock
    else if query_lower.contains("unlock") && query_lower.contains("bootloader") {
        commands.push("fastboot flashing unlock".to_string());
        commands.push("fastboot oem unlock".to_string());
        explanation = "Attempting bootloader unlock (Standard & OEM commands). WARNING: Wipes data.".to_string();
        confidence = 0.9;
    }

    // Fallback
    if commands.is_empty() {
        explanation = "I'm not sure how to translate that yet. Try 'reboot recovery', 'wipe data', or 'flash boot'.".to_string();
        confidence = 0.1;
    }

    Ok(ParsedCommand {
        original: query,
        commands,
        confidence,
        explanation,
    })
}

#[command]
pub async fn execute_cli_command(command: String) -> Result<String, String> {
    // Security check: Only allow adb and fastboot commands
    if !command.starts_with("adb") && !command.starts_with("fastboot") {
        return Err("Security Error: Only ADB and Fastboot commands are allowed.".to_string());
    }

    // Split command into parts
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        return Err("Empty command".to_string());
    }

    let bin = parts[0];
    let args = &parts[1..];

    let output = Command::new(bin)
        .args(args)
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
