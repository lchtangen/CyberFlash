use tauri::{command, AppHandle, Manager};
use serde::{Serialize, Deserialize};
use reqwest::header::{AUTHORIZATION, USER_AGENT, ACCEPT};
use tauri_plugin_shell::ShellExt;

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubArtifact {
    pub id: u64,
    pub name: String,
    pub size_in_bytes: u64,
    pub created_at: String,
    pub download_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubWorkflowRun {
    pub id: u64,
    pub name: String,
    pub status: String,
    pub conclusion: Option<String>,
    pub created_at: String,
}

#[command]
pub async fn list_github_artifacts(token: String, owner: String, repo: String) -> Result<Vec<GithubArtifact>, String> {
    let client = reqwest::Client::new();
    let url = format!("https://api.github.com/repos/{}/{}/actions/artifacts", owner, repo);

    let resp = client.get(&url)
        .header(USER_AGENT, "CyberFlash-V2")
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header(ACCEPT, "application/vnd.github.v3+json")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("GitHub API Error: {}", resp.status()));
    }

    let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
    
    let mut artifacts = Vec::new();
    if let Some(list) = json["artifacts"].as_array() {
        for item in list {
            artifacts.push(GithubArtifact {
                id: item["id"].as_u64().unwrap_or(0),
                name: item["name"].as_str().unwrap_or("unknown").to_string(),
                size_in_bytes: item["size_in_bytes"].as_u64().unwrap_or(0),
                created_at: item["created_at"].as_str().unwrap_or("").to_string(),
                download_url: item["archive_download_url"].as_str().unwrap_or("").to_string(),
            });
        }
    }

    Ok(artifacts)
}

#[command]
pub async fn list_recent_runs(token: String, owner: String, repo: String) -> Result<Vec<GithubWorkflowRun>, String> {
    let client = reqwest::Client::new();
    let url = format!("https://api.github.com/repos/{}/{}/actions/runs?per_page=5", owner, repo);

    let resp = client.get(&url)
        .header(USER_AGENT, "CyberFlash-V2")
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header(ACCEPT, "application/vnd.github.v3+json")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("GitHub API Error: {}", resp.status()));
    }

    let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
    
    let mut runs = Vec::new();
    if let Some(list) = json["workflow_runs"].as_array() {
        for item in list {
            runs.push(GithubWorkflowRun {
                id: item["id"].as_u64().unwrap_or(0),
                name: item["name"].as_str().unwrap_or("unknown").to_string(),
                status: item["status"].as_str().unwrap_or("unknown").to_string(),
                conclusion: item["conclusion"].as_str().map(|s| s.to_string()),
                created_at: item["created_at"].as_str().unwrap_or("").to_string(),
            });
        }
    }

    Ok(runs)
}

#[command]
pub async fn backup_app_data(app: AppHandle, package: String) -> Result<String, String> {
    let download_dir = app.path().download_dir().map_err(|e| e.to_string())?;
    let backup_file = download_dir.join(format!("{}_backup.ab", package));
    
    // Note: This blocks until user confirms on device
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["backup", "-f", backup_file.to_string_lossy().as_ref(), "-apk", &package])
        .output()
        .await
        .map_err(|e| e.to_string())?;
        
    if output.status.success() {
        Ok(backup_file.to_string_lossy().to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[command]
pub async fn restore_app_data(app: AppHandle, backup_path: String) -> Result<String, String> {
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["restore", &backup_path])
        .output()
        .await
        .map_err(|e| e.to_string())?;
        
    if output.status.success() {
        Ok("Restore initiated. Please confirm on device.".to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
