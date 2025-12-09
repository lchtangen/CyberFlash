use tauri::{AppHandle, Emitter};
use std::process::Command;
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use serde::Serialize;
use crate::commands::scheduler;

static MONITOR_RUNNING: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Serialize, Clone)]
pub struct DeviceStatus {
    pub serial: String,
    pub state: String, // "device", "recovery", "sideload", "fastboot"
    pub connection_type: String, // "adb" or "fastboot"
}

#[tauri::command]
pub fn start_usb_monitor(app: AppHandle) {
    if MONITOR_RUNNING.load(Ordering::SeqCst) {
        return;
    }
    MONITOR_RUNNING.store(true, Ordering::SeqCst);

    thread::spawn(move || {
        loop {
            let mut devices = Vec::new();

            // Check ADB Devices
            if let Ok(output) = Command::new("adb").arg("devices").output() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                for line in output_str.lines().skip(1) {
                    if line.trim().is_empty() { continue; }
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        let serial = parts[0].to_string();
                        
                        // Check if this is a new device (simple logic for now)
                        // In a real app, we'd keep a Set of known devices
                        // For now, we just trigger the scheduler check asynchronously
                        let app_handle = app.clone();
                        let serial_clone = serial.clone();
                        tauri::async_runtime::spawn(async move {
                            scheduler::check_and_run_schedules(app_handle, serial_clone).await;
                        });

                        devices.push(DeviceStatus {
                            serial,
                            state: parts[1].to_string(),
                            connection_type: "adb".to_string(),
                        });
                    }
                }
            }

            // Check Fastboot Devices
            if let Ok(output) = Command::new("fastboot").arg("devices").output() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                for line in output_str.lines() {
                    if line.trim().is_empty() { continue; }
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        // Avoid duplicates if somehow both show up
                        if !devices.iter().any(|d| d.serial == parts[0]) {
                            devices.push(DeviceStatus {
                                serial: parts[0].to_string(),
                                state: "fastboot".to_string(),
                                connection_type: "fastboot".to_string(),
                            });
                        }
                    }
                }
            }

            // Emit event
            if let Err(e) = app.emit("device-status-update", &devices) {
                eprintln!("Failed to emit device status: {}", e);
            }

            thread::sleep(Duration::from_secs(1));
        }
    });
}
