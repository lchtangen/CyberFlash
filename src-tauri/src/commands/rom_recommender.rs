use tauri::command;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct RomRecommendation {
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub official_url: String,
    pub match_score: u8,
}

#[command]
pub async fn recommend_roms(usage_pattern: String) -> Result<Vec<RomRecommendation>, String> {
    let mut recommendations = Vec::new();

    match usage_pattern.as_str() {
        "gaming" => {
            recommendations.push(RomRecommendation {
                name: "RisingOS".to_string(),
                description: "Performance-oriented with heavy customization and gaming modes.".to_string(),
                tags: vec!["Performance".to_string(), "Customization".to_string()],
                official_url: "https://risingos.org".to_string(),
                match_score: 95,
            });
            recommendations.push(RomRecommendation {
                name: "Evolution X".to_string(),
                description: "Pixel feel with gaming optimizations and customization.".to_string(),
                tags: vec!["Pixel-UI".to_string(), "Gaming".to_string()],
                official_url: "https://evolution-x.org".to_string(),
                match_score: 90,
            });
            recommendations.push(RomRecommendation {
                name: "crDroid".to_string(),
                description: "High stability with tons of gaming-friendly tweaks.".to_string(),
                tags: vec!["Tweaks".to_string(), "Stability".to_string()],
                official_url: "https://crdroid.net".to_string(),
                match_score: 85,
            });
        },
        "battery" => {
            recommendations.push(RomRecommendation {
                name: "LineageOS".to_string(),
                description: "The gold standard for stability and battery life. No bloat.".to_string(),
                tags: vec!["Stable".to_string(), "Clean".to_string()],
                official_url: "https://lineageos.org".to_string(),
                match_score: 98,
            });
            recommendations.push(RomRecommendation {
                name: "PixelExperience".to_string(),
                description: "Stock Pixel software. Optimized for daily use.".to_string(),
                tags: vec!["Stock".to_string(), "Simple".to_string()],
                official_url: "https://get.pixelexperience.org".to_string(),
                match_score: 85,
            });
        },
        "privacy" => {
            recommendations.push(RomRecommendation {
                name: "CalyxOS".to_string(),
                description: "Privacy by design. De-googled but usable.".to_string(),
                tags: vec!["Privacy".to_string(), "Security".to_string()],
                official_url: "https://calyxos.org".to_string(),
                match_score: 99,
            });
            recommendations.push(RomRecommendation {
                name: "/e/OS".to_string(),
                description: "Completely de-googled ecosystem.".to_string(),
                tags: vec!["De-Googled".to_string(), "Privacy".to_string()],
                official_url: "https://e.foundation".to_string(),
                match_score: 95,
            });
             recommendations.push(RomRecommendation {
                name: "GrapheneOS".to_string(),
                description: "Hardened Android. The most secure option (Pixel only).".to_string(),
                tags: vec!["Hardened".to_string(), "Security".to_string()],
                official_url: "https://grapheneos.org".to_string(),
                match_score: 90,
            });
        },
        _ => {
            // Balanced / Default
             recommendations.push(RomRecommendation {
                name: "LineageOS".to_string(),
                description: "Balanced performance and battery.".to_string(),
                tags: vec!["Stable".to_string()],
                official_url: "https://lineageos.org".to_string(),
                match_score: 90,
            });
             recommendations.push(RomRecommendation {
                name: "Paranoid Android".to_string(),
                description: "Beautiful design and smooth performance.".to_string(),
                tags: vec!["Aesthetics".to_string(), "Smooth".to_string()],
                official_url: "https://paranoidandroid.co".to_string(),
                match_score: 88,
            });
        }
    }

    Ok(recommendations)
}
