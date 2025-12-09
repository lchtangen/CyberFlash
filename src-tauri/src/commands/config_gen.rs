use tauri::command;
use serde::{Serialize, Deserialize};
use crate::commands::flash_as_code::{FlashConfig, FlashStep};
use regex::Regex;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScrapedData {
    pub title: String,
    pub possible_rom_links: Vec<String>,
    pub possible_recovery_links: Vec<String>,
    pub detected_device: Option<String>,
}

#[command]
pub async fn generate_config_from_url(url: String) -> Result<String, String> {
    // 1. Fetch the content (Simulated for now to ensure reliability in this env, 
    //    but structure allows for real reqwest call)
    let html_content = fetch_url_content(&url).await.map_err(|e| e.to_string())?;

    // 2. Analyze the content
    let data = analyze_html(&html_content);

    // 3. Build the FlashConfig
    let config = build_config_from_data(data);

    // 4. Serialize to YAML
    let yaml = serde_yaml::to_string(&config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;

    Ok(yaml)
}

async fn fetch_url_content(url: &str) -> Result<String, reqwest::Error> {
    // In a real production build, we would use:
    // reqwest::get(url).await?.text().await
    
    // For this demo/environment, we'll return a mock XDA thread HTML if the URL is a placeholder,
    // otherwise try a real fetch.
    if url.contains("mock-xda") {
        return Ok(r#"
            <html>
                <head><title>[ROM][14.0][OnePlus 7 Pro] LineageOS 21 [OFFICIAL]</title></head>
                <body>
                    <h1>LineageOS 21 for OnePlus 7 Pro</h1>
                    <p>Maintained by: SoulMaster</p>
                    
                    <h2>Downloads</h2>
                    <a href="https://mirrorbits.lineageos.org/full/guacamole/20240101/lineage-21.0-20240101-nightly-guacamole-signed.zip">Download ROM</a>
                    <a href="https://twrp.me/oneplus/twrp-3.7.0_9-0-guacamole.img">Download Recovery</a>
                    
                    <h2>Instructions</h2>
                    <ol>
                        <li>Reboot to recovery</li>
                        <li>Format Data</li>
                        <li>Flash ROM zip</li>
                        <li>Reboot</li>
                    </ol>
                </body>
            </html>
        "#.to_string());
    }

    // Attempt real fetch with short timeout
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .user_agent("CyberFlash-V2/1.0")
        .build()?;
        
    client.get(url).send().await?.text().await
}

fn analyze_html(html: &str) -> ScrapedData {
    let title_re = Regex::new(r"<title>(.*?)</title>").unwrap();
    let link_re = Regex::new(r#"href=["'](.*?)["']"#).unwrap();
    
    let title = title_re.captures(html)
        .map(|c| c.get(1).unwrap().as_str().to_string())
        .unwrap_or("Unknown ROM".to_string());

    let mut possible_rom_links = Vec::new();
    let mut possible_recovery_links = Vec::new();

    for cap in link_re.captures_iter(html) {
        let link = cap.get(1).unwrap().as_str();
        if link.ends_with(".zip") {
            possible_rom_links.push(link.to_string());
        } else if link.ends_with(".img") {
            possible_recovery_links.push(link.to_string());
        }
    }

    let detected_device = if title.to_lowercase().contains("oneplus 7 pro") || html.to_lowercase().contains("guacamole") {
        Some("OnePlus 7 Pro".to_string())
    } else if title.to_lowercase().contains("pixel") {
        Some("Google Pixel".to_string())
    } else {
        None
    };

    ScrapedData {
        title,
        possible_rom_links,
        possible_recovery_links,
        detected_device,
    }
}

fn build_config_from_data(data: ScrapedData) -> FlashConfig {
    let mut steps = Vec::new();

    // Step 1: Wipe (Standard practice)
    steps.push(FlashStep::Wipe {
        partitions: vec!["userdata".to_string(), "cache".to_string()] // Safe defaults
    });

    // Step 2: Flash Recovery if found
    if let Some(rec_link) = data.possible_recovery_links.first() {
        steps.push(FlashStep::FlashRecovery {
            file: rec_link.clone()
        });
        steps.push(FlashStep::Reboot { mode: "recovery".to_string() });
    }

    // Step 3: Flash ROM
    if let Some(rom_link) = data.possible_rom_links.first() {
        // Heuristic: If it's a zip, it might be sideload or flash
        // We'll default to Sideload for safety as it works in more recoveries
        steps.push(FlashStep::Sideload {
            file: rom_link.clone()
        });
    } else {
        // Placeholder if no link found
        steps.push(FlashStep::Sideload {
            file: "INSERT_ROM_URL_HERE.zip".to_string()
        });
    }

    // Step 4: Reboot
    steps.push(FlashStep::Reboot { mode: "system".to_string() });

    FlashConfig {
        name: data.title,
        device: data.detected_device.unwrap_or("Generic Device".to_string()),
        version: "Auto-Generated".to_string(),
        steps,
    }
}
