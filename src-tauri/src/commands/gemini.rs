use tauri::command;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize, Deserialize)]
struct Part {
    text: String,
}

#[derive(Serialize, Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Serialize, Deserialize)]
struct Candidate {
    content: Content,
}

pub async fn call_gemini_api(prompt: String, api_key: String, model: Option<String>) -> Result<String, String> {
    if api_key.is_empty() {
        return Err("API Key is missing.".to_string());
    }

    let model_name = model.unwrap_or_else(|| "gemini-1.5-flash".to_string());
    let client = Client::new();
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model_name, api_key
    );

    let request_body = json!({
        "contents": [{
            "parts": [{
                "text": prompt
            }]
        }]
    });

    let response = client
        .post(&url)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("API Error: {}", response.status()));
    }

    let gemini_response: GeminiResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    if let Some(candidate) = gemini_response.candidates.first() {
        if let Some(part) = candidate.content.parts.first() {
            return Ok(part.text.clone());
        }
    }

    Err("No response generated.".to_string())
}

#[command]
pub async fn ask_gemini(prompt: String, api_key: String, model: Option<String>, context: Option<String>) -> Result<String, String> {
    let system_instruction = "You are CyberFlash AI, an expert Android Flashing Assistant. 
    Your goal is to help users flash Custom ROMs, recover bricked devices, and use ADB/Fastboot tools safely.
    
    Guidelines:
    1. Be concise, technical, and safety-conscious.
    2. Always warn about data loss before suggesting wipe commands.
    3. If the user asks to flash, verify battery level (if known) is >30%.
    4. Use standard ADB/Fastboot syntax.
    5. If the device is offline, guide the user to enable USB Debugging.
    6. Do not hallucinate features not present in standard Android tools.
    
    Current Device Context:
    ";

    let full_prompt = if let Some(ctx) = context {
        format!("{}\n{}\n\nUser Query: {}", system_instruction, ctx, prompt)
    } else {
        format!("{}\nNo device connected.\n\nUser Query: {}", system_instruction, prompt)
    };

    call_gemini_api(full_prompt, api_key, model).await
}
