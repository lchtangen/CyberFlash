use tauri::command;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslatedText {
    pub original: String,
    pub translated: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[command]
pub async fn translate_image(_image_data: String) -> Result<Vec<TranslatedText>, String> {
    // In a real app, we would decode the base64 image and send it to Gemini Vision or Tesseract.
    // Here we simulate finding Chinese text and translating it.
    
    // Simulate processing delay
    tokio::time::sleep(std::time::Duration::from_millis(1500)).await;

    // Mock response
    Ok(vec![
        TranslatedText {
            original: "重启系统".to_string(),
            translated: "Reboot System".to_string(),
            x: 50,
            y: 100,
            width: 200,
            height: 40,
        },
        TranslatedText {
            original: "清除数据".to_string(),
            translated: "Wipe Data".to_string(),
            x: 50,
            y: 160,
            width: 200,
            height: 40,
        },
        TranslatedText {
            original: "安装更新".to_string(),
            translated: "Install Update".to_string(),
            x: 50,
            y: 220,
            width: 200,
            height: 40,
        }
    ])
}
