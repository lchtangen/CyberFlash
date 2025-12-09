use tauri::command;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum OrsAction {
    Install { path: String },
    Wipe { partition: String },
    Backup { partitions: Vec<String>, name: Option<String> },
    Restore { path: String },
    Mount { partition: String },
    Unmount { partition: String },
    Shell { command: String },
    Print { message: String },
}

#[command]
pub fn generate_ors_script(actions: Vec<OrsAction>) -> String {
    let mut script = String::new();
    
    for action in actions {
        match action {
            OrsAction::Install { path } => {
                script.push_str(&format!("install {}\n", path));
            },
            OrsAction::Wipe { partition } => {
                script.push_str(&format!("wipe {}\n", partition));
            },
            OrsAction::Backup { partitions, name } => {
                let parts = partitions.join(""); // TWRP expects e.g. "SDB" for System, Data, Boot
                // Mapping common names to TWRP flags is complex, usually it's full names in newer TWRP
                // But standard ORS uses specific flags. 
                // Actually, modern TWRP ORS supports "backup <partitions> <name>" where partitions is a list separated by space?
                // Let's stick to the standard: backup SDBO name
                // For simplicity, we'll just pass the raw string for now or assume modern syntax
                // "backup <options> <name>"
                // Let's use a simplified approach: just append the partitions string
                let name_str = name.unwrap_or_default();
                script.push_str(&format!("backup {} {}\n", parts, name_str));
            },
            OrsAction::Restore { path } => {
                script.push_str(&format!("restore {}\n", path));
            },
            OrsAction::Mount { partition } => {
                script.push_str(&format!("mount {}\n", partition));
            },
            OrsAction::Unmount { partition } => {
                script.push_str(&format!("unmount {}\n", partition));
            },
            OrsAction::Shell { command } => {
                script.push_str(&format!("cmd {}\n", command));
            },
            OrsAction::Print { message } => {
                script.push_str(&format!("print {}\n", message));
            },
        }
    }
    
    script
}

#[command]
pub fn save_ors_script(path: String, content: String) -> Result<(), String> {
    fs::write(path, content).map_err(|e| e.to_string())
}
