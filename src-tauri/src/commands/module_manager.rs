use tauri::{command, AppHandle};
use serde::{Deserialize, Serialize};
use crate::commands::adb;

#[derive(Debug, Serialize, Deserialize)]
pub struct MagiskModule {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub enabled: bool,
}

#[command]
pub async fn list_magisk_modules(app: AppHandle) -> Result<Vec<MagiskModule>, String> {
    // Check root first
    let root_status = adb::check_root_status(app.clone()).await?;
    if !root_status {
        return Err("Device is not rooted or root access denied.".into());
    }

    // List directories in /data/adb/modules
    let output = adb::run_adb_shell(app.clone(), "ls -1 /data/adb/modules".into()).await?;
    let module_ids: Vec<&str> = output.trim().split('\n').filter(|s| !s.is_empty()).collect();
    
    let mut modules = Vec::new();

    for id in module_ids {
        // Read module.prop
        let prop_content = adb::run_adb_shell(app.clone(), format!("cat /data/adb/modules/{}/module.prop", id)).await.unwrap_or_default();
        
        // Check if disabled file exists
        let disabled_check = adb::run_adb_shell(app.clone(), format!("ls /data/adb/modules/{}/disable", id)).await;
        let enabled = disabled_check.is_err() || disabled_check.unwrap().contains("No such file");

        let mut name = String::new();
        let mut version = String::new();
        let mut author = String::new();
        let mut description = String::new();

        for line in prop_content.lines() {
            if line.starts_with("name=") { name = line.replace("name=", ""); }
            if line.starts_with("version=") { version = line.replace("version=", ""); }
            if line.starts_with("author=") { author = line.replace("author=", ""); }
            if line.starts_with("description=") { description = line.replace("description=", ""); }
        }

        if !name.is_empty() {
            modules.push(MagiskModule {
                id: id.to_string(),
                name,
                version,
                author,
                description,
                enabled,
            });
        }
    }

    Ok(modules)
}

#[command]
pub async fn toggle_module(app: AppHandle, id: String, enable: bool) -> Result<String, String> {
    if enable {
        adb::run_adb_shell(app, format!("rm /data/adb/modules/{}/disable", id)).await
    } else {
        adb::run_adb_shell(app, format!("touch /data/adb/modules/{}/disable", id)).await
    }
}

#[command]
pub async fn remove_module(app: AppHandle, id: String) -> Result<String, String> {
    // Magisk way: touch remove file, it will be removed on reboot
    adb::run_adb_shell(app, format!("touch /data/adb/modules/{}/remove", id)).await
}

#[command]
pub async fn install_module_zip(app: AppHandle, zip_path: String) -> Result<String, String> {
    // Push zip to /sdcard/
    let file_name = std::path::Path::new(&zip_path).file_name().unwrap().to_string_lossy();
    let remote_path = format!("/sdcard/{}", file_name);
    
    adb::push_file(app.clone(), zip_path, remote_path.clone()).await?;
    
    // Install via magisk cli
    // 'magisk --install-module' might not be available in all versions, usually we use 'magisk --install-module <zip>'
    // Or 'su -c magisk --install-module ...'
    
    let cmd = format!("magisk --install-module {}", remote_path);
    adb::run_adb_shell(app, cmd).await
}
