use tauri::{command, AppHandle};
use tauri_plugin_shell::ShellExt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TuningRecommendation {
    pub category: String, // "Kernel", "Network", "UI"
    pub setting: String,
    pub current_value: String,
    pub suggested_value: String,
    pub reason: String,
    pub impact: String, // "High", "Medium", "Low"
}

async fn run_adb_shell(app: &AppHandle, cmd: &str) -> Result<String, String> {
    let output = app.shell().sidecar("adb")
        .map_err(|e| e.to_string())?
        .args(["shell", cmd])
        .output()
        .await
        .map_err(|e| e.to_string())?;
        
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[command]
pub async fn analyze_performance(app: AppHandle) -> Result<Vec<TuningRecommendation>, String> {
    let mut recommendations = Vec::new();

    // 1. Check Window Animation Scale
    let anim_scale = run_adb_shell(&app, "settings get global window_animation_scale").await.unwrap_or("1.0".to_string());
    if anim_scale != "0.5" && anim_scale != "0" {
        recommendations.push(TuningRecommendation {
            category: "UI".to_string(),
            setting: "Window Animation Scale".to_string(),
            current_value: anim_scale,
            suggested_value: "0.5".to_string(),
            reason: "Makes the UI feel snappier.".to_string(),
            impact: "High".to_string(),
        });
    }

    // 2. Check TCP Congestion Control (Kernel)
    // Requires root usually to read /proc/sys/net/ipv4/tcp_congestion_control
    // We'll try, if fails, ignore.
    if let Ok(tcp) = run_adb_shell(&app, "cat /proc/sys/net/ipv4/tcp_congestion_control").await {
        if !tcp.contains("bbr") {
             recommendations.push(TuningRecommendation {
                category: "Kernel".to_string(),
                setting: "TCP Congestion Control".to_string(),
                current_value: tcp,
                suggested_value: "bbr".to_string(),
                reason: "BBR provides better throughput on unstable networks.".to_string(),
                impact: "Medium".to_string(),
            });
        }
    }

    // 3. Check Logcat Buffer
    // getprop ro.logd.size
    let log_size = run_adb_shell(&app, "getprop ro.logd.size").await.unwrap_or("256k".to_string());
    if log_size != "64k" && log_size != "off" {
        recommendations.push(TuningRecommendation {
            category: "System".to_string(),
            setting: "Logcat Buffer".to_string(),
            current_value: log_size,
            suggested_value: "64k".to_string(),
            reason: "Reduces background logging overhead.".to_string(),
            impact: "Low".to_string(),
        });
    }

    Ok(recommendations)
}

#[command]
pub async fn apply_tuning(app: AppHandle, setting: String, value: String) -> Result<String, String> {
    match setting.as_str() {
        "Window Animation Scale" => {
            run_adb_shell(&app, &format!("settings put global window_animation_scale {}", value)).await?;
            run_adb_shell(&app, &format!("settings put global transition_animation_scale {}", value)).await?;
            run_adb_shell(&app, &format!("settings put global animator_duration_scale {}", value)).await?;
            Ok("Applied UI animations".to_string())
        },
        "TCP Congestion Control" => {
            // Requires root
            run_adb_shell(&app, &format!("su -c 'echo {} > /proc/sys/net/ipv4/tcp_congestion_control'", value)).await?;
            Ok("Applied TCP Congestion Control".to_string())
        },
        "Logcat Buffer" => {
            // Requires root or special prop set
            // Usually ro props are read only, but we can try logd.logpersistd.size or similar
            // Or just use logcat -G
            run_adb_shell(&app, &format!("logcat -G {}", value)).await?;
            Ok("Applied Logcat Buffer Size".to_string())
        },
        _ => Err("Unknown setting".to_string())
    }
}
