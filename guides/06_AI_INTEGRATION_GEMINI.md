# 🤖 AI Integration Guide: Google Gemini 3 Pro

**Version**: 1.0.0 | **Model**: Gemini 1.5 Pro / 3 Pro (Preview) | **Status**: Beta

---

## 🧠 The "Brain" of CyberFlash

CyberFlash V2 integrates **Google Gemini 3 Pro** to provide intelligent assistance, error prediction, and interactive help.

### Key AI Features
1.  **Predictive Flashing**: Analyzes device state (battery, storage, model) to predict success probability before starting.
2.  **Smart Error Recovery**: If an ADB command fails, Gemini analyzes the error log and suggests the exact fix (e.g., "Install drivers", "Unlock bootloader").
3.  **Interactive Assistant**: A chat interface for users to ask questions like "How do I back up my data?" or "Is this ROM stable?".

---

## 🛠️ Implementation Architecture

We do **not** call the Gemini API directly from the Vue frontend (to protect API keys). All calls go through the Rust backend.

### 1. Rust Backend (Secure Proxy)

```rust
// src-tauri/src/commands/gemini.rs
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
pub struct GeminiRequest {
    prompt: String,
    context: String, // Device logs, error messages
}

#[tauri::command]
pub async fn ask_gemini(request: GeminiRequest) -> Result<String, String> {
    let api_key = env::var("GEMINI_API_KEY").map_err(|_| "API Key not found")?;
    let client = Client::new();

    let response = client
        .post("https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent")
        .query(&[("key", api_key)])
        .json(&serde_json::json!({
            "contents": [{
                "parts": [{
                    "text": format!("Context: {}\n\nUser Question: {}", request.context, request.prompt)
                }]
            }]
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    // Parse response...
    Ok("Parsed AI Response".to_string())
}
```

### 2. Vue Frontend (Composable)

```typescript
// src/composables/useGemini.ts
import { invoke } from '@tauri-apps/api/core'
import { ref } from 'vue'

export function useGemini() {
  const loading = ref(false)
  const response = ref('')

  async function ask(prompt: string, context: string = '') {
    loading.value = true
    try {
      response.value = await invoke('ask_gemini', { 
        request: { prompt, context } 
      })
    } catch (e) {
      console.error(e)
      response.value = "Sorry, I couldn't reach the AI brain."
    } finally {
      loading.value = false
    }
  }

  return { ask, response, loading }
}
```

---

## 📝 Prompt Engineering Strategy

To get the best results, we use structured system prompts.

### System Prompt Template
```text
You are CyberFlash AI, an expert Android ROM flashing assistant.
Your goal is to help users flash crDroid onto a OnePlus 7 Pro safely.
- Be concise and technical but accessible.
- If an error occurs, analyze the log and provide a step-by-step fix.
- Warn users about data loss before dangerous operations.
- Do not hallucinate commands. Only recommend standard ADB/Fastboot commands.
```

### Context Injection
When sending a request, we automatically inject:
- **Device Model**: `OnePlus 7 Pro (GM1917)`
- **Battery Level**: `85%`
- **Current Phase**: `Phase 3: Bootloader Unlock`
- **Last Error**: `FAILED (remote: 'unknown command')`

---

## 🔒 Security Best Practices

1.  **Never expose API keys in frontend code.**
2.  **Rate Limiting**: Implement a token bucket in Rust to prevent abuse.
3.  **Sanitization**: Strip sensitive user data (IMEI, serial numbers) from logs before sending to Gemini.
