use tauri::command;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SentimentReport {
    pub score: i32, // 0-100
    pub verdict: String, // "Stable", "Mixed", "Unstable"
    pub positive_count: usize,
    pub negative_count: usize,
    pub key_issues: Vec<String>,
}

#[command]
pub async fn analyze_sentiment(url: String) -> Result<SentimentReport, String> {
    // 1. Fetch content (Simulated for reliability)
    let content = fetch_content(&url).await;
    
    // 2. Analyze
    let (pos, neg, issues) = analyze_text(&content);
    
    // 3. Calculate Score
    let total = pos + neg;
    let score = if total == 0 {
        50 // Neutral
    } else {
        ((pos as f32 / total as f32) * 100.0) as i32
    };
    
    let verdict = if score >= 80 {
        "Stable".to_string()
    } else if score >= 50 {
        "Mixed".to_string()
    } else {
        "Unstable".to_string()
    };

    Ok(SentimentReport {
        score,
        verdict,
        positive_count: pos,
        negative_count: neg,
        key_issues: issues,
    })
}

async fn fetch_content(url: &str) -> String {
    // Mock content based on URL for demo
    if url.contains("buggy") {
        return "User1: This ROM is full of bugs. Camera crashes. Bootloops often. Not recommended.".to_string();
    } else if url.contains("stable") {
        return "User1: Amazing work! Very smooth. Battery life is great. No bugs found so far.".to_string();
    }
    
    // Default mixed content
    "User1: Good ROM but battery is average. User2: Camera works fine. User3: I had a random reboot once.".to_string()
}

fn analyze_text(text: &str) -> (usize, usize, Vec<String>) {
    let text_lower = text.to_lowercase();
    
    let pos_keywords = ["smooth", "great", "stable", "fast", "working", "good", "amazing", "perfect"];
    let neg_keywords = ["bug", "crash", "bootloop", "lag", "drain", "broken", "fail", "error", "reboot"];
    
    let mut pos_count = 0;
    let mut neg_count = 0;
    let mut issues = Vec::new();
    
    for word in pos_keywords {
        pos_count += text_lower.matches(word).count();
    }
    
    for word in neg_keywords {
        let count = text_lower.matches(word).count();
        if count > 0 {
            neg_count += count;
            issues.push(word.to_string());
        }
    }
    
    // Deduplicate issues
    issues.sort();
    issues.dedup();
    
    (pos_count, neg_count, issues)
}
