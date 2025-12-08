use tauri::Emitter;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::thread;

mod commands;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn run_adb_command(args: Vec<String>) -> Result<String, String> {
    let output = Command::new("adb")
        .args(args)
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[tauri::command]
fn start_logcat(app_handle: tauri::AppHandle) {
    thread::spawn(move || {
        let mut child = Command::new("adb")
            .args(&["logcat", "-v", "brief"])
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start logcat");

        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(l) = line {
                    // Emit event to frontend
                    let _ = app_handle.emit("logcat-data", l);
                }
            }
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            run_adb_command,
            start_logcat,
            commands::adb::get_connected_devices,
            commands::fastboot::get_fastboot_devices,
            commands::gemini::ask_gemini,
            commands::automation::start_flash_process
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
