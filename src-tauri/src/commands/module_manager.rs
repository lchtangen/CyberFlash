use tauri::{command, AppHandle, Manager};
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
        let enabled = match disabled_check {
            Ok(output) => !output.contains("No such file"),
            Err(_) => true, // If ls fails (e.g. file not found), assume enabled
        };

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

#[command]
pub async fn get_denylist(app: AppHandle) -> Result<Vec<String>, String> {
    let output = adb::run_adb_shell(app, "magisk --denylist ls".into()).await?;
    Ok(output.lines().map(|s| s.to_string()).collect())
}

#[command]
pub async fn toggle_denylist(app: AppHandle, package: String, enable: bool) -> Result<String, String> {
    let cmd = if enable {
        format!("magisk --denylist add {}", package)
    } else {
        format!("magisk --denylist rm {}", package)
    };
    adb::run_adb_shell(app, cmd).await
}

#[command]
pub async fn backup_modules(app: AppHandle) -> Result<String, String> {
    // Strategy: 
    // 1. su -c "tar -czf /sdcard/modules_backup.tar.gz /data/adb/modules"
    // 2. adb pull /sdcard/modules_backup.tar.gz
    // 3. su -c "rm /sdcard/modules_backup.tar.gz"
    
    let tar_cmd = "tar -czf /sdcard/modules_backup.tar.gz /data/adb/modules";
    adb::run_adb_shell(app.clone(), format!("su -c '{}'", tar_cmd)).await?;
    
    let download_dir = app.path().download_dir().map_err(|e| e.to_string())?;
    let local_path = download_dir.join(format!("magisk_modules_backup_{}.tar.gz", chrono::Local::now().format("%Y%m%d_%H%M%S")));
    
    adb::pull_file(app.clone(), "/sdcard/modules_backup.tar.gz".into(), local_path.to_string_lossy().to_string()).await?;
    
    adb::run_adb_shell(app.clone(), "rm /sdcard/modules_backup.tar.gz".into()).await?;
    
    Ok(local_path.to_string_lossy().to_string())
}

#[command]
pub async fn remove_all_modules(app: AppHandle) -> Result<String, String> {
    // Mark all modules for removal
    let cmd = "find /data/adb/modules -maxdepth 1 -mindepth 1 -type d -exec touch {}/remove \\;";
    adb::run_adb_shell(app, format!("su -c '{}'", cmd)).await
}
