use tauri::command;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PredictionResult {
    pub score: u8, // 0-100
    pub risk_level: String, // "Low", "Medium", "High", "Critical"
    pub factors: Vec<RiskFactor>,
    pub recommendation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RiskFactor {
    pub name: String,
    pub impact: i8, // Negative or positive impact on score
    pub description: String,
}

#[command]
pub async fn calculate_success_score(
    battery_level: u8,
    is_bootloader_unlocked: bool,
    is_debug_enabled: bool,
    firmware_matched: bool,
) -> Result<PredictionResult, String> {
    let mut score: i16 = 100;
    let mut factors = Vec::new();

    // 1. Battery Analysis
    if battery_level < 20 {
        score -= 50;
        factors.push(RiskFactor {
            name: "Critical Battery".to_string(),
            impact: -50,
            description: "Battery is below 20%. Flashing is highly dangerous.".to_string(),
        });
    } else if battery_level < 50 {
        score -= 20;
        factors.push(RiskFactor {
            name: "Low Battery".to_string(),
            impact: -20,
            description: "Battery is below 50%. Charge recommended.".to_string(),
        });
    } else {
        factors.push(RiskFactor {
            name: "Good Battery".to_string(),
            impact: 10,
            description: "Battery level is sufficient for operations.".to_string(),
        });
    }

    // 2. Bootloader Status
    if !is_bootloader_unlocked {
        score -= 80;
        factors.push(RiskFactor {
            name: "Bootloader Locked".to_string(),
            impact: -80,
            description: "Bootloader is locked. Flashing custom ROMs will fail.".to_string(),
        });
    } else {
        factors.push(RiskFactor {
            name: "Bootloader Unlocked".to_string(),
            impact: 10,
            description: "Device is ready for custom firmware.".to_string(),
        });
    }

    // 3. USB Debugging
    if !is_debug_enabled {
        score -= 30;
        factors.push(RiskFactor {
            name: "USB Debugging Off".to_string(),
            impact: -30,
            description: "ADB commands may be restricted.".to_string(),
        });
    }

    // 4. Firmware Match
    if !firmware_matched {
        score -= 40;
        factors.push(RiskFactor {
            name: "Firmware Mismatch".to_string(),
            impact: -40,
            description: "Target ROM may not match current firmware base.".to_string(),
        });
    }

    // Clamp score
    let final_score = score.clamp(0, 100) as u8;

    let risk_level = match final_score {
        0..=40 => "Critical",
        41..=70 => "High",
        71..=89 => "Medium",
        _ => "Low",
    }.to_string();

    let recommendation = match final_score {
        0..=40 => "DO NOT FLASH. Resolve critical issues first.",
        41..=70 => "Proceed with extreme caution. Backup data immediately.",
        71..=89 => "Looks good, but double-check battery and backups.",
        _ => "System optimal. Ready to flash.",
    }.to_string();

    Ok(PredictionResult {
        score: final_score,
        risk_level,
        factors,
        recommendation,
    })
}
