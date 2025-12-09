use tauri::{command, State, Emitter, AppHandle};
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use chrono::Local;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActivityItem {
    pub id: i64,
    pub action: String,
    pub details: Option<String>,
    pub status: String,
    pub timestamp: String,
    pub device_serial: Option<String>,
}

pub struct HistoryDb(pub Mutex<Connection>);

impl HistoryDb {
    pub fn init() -> Result<Self> {
        let path = "history.db"; // In production, use app_data_dir
        let conn = Connection::open(path)?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS activity_log (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                action TEXT NOT NULL,
                details TEXT,
                status TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                device_serial TEXT
            )",
            [],
        )?;
        
        Ok(HistoryDb(Mutex::new(conn)))
    }
}

#[command]
pub async fn add_activity(
    app: AppHandle,
    db: State<'_, HistoryDb>,
    action: String,
    details: Option<String>,
    status: String,
    device_serial: Option<String>,
) -> Result<i64, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    conn.execute(
        "INSERT INTO activity_log (action, details, status, timestamp, device_serial) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![action, details, status, timestamp, device_serial],
    ).map_err(|e| e.to_string())?;
    
    let id = conn.last_insert_rowid();

    // Emit event for real-time updates
    let item = ActivityItem {
        id,
        action,
        details,
        status,
        timestamp,
        device_serial,
    };
    let _ = app.emit("history:update", item);

    Ok(id)
}

#[command]
pub async fn get_activity_log(db: State<'_, HistoryDb>, limit: Option<u32>) -> Result<Vec<ActivityItem>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let limit = limit.unwrap_or(50);
    
    let mut stmt = conn.prepare(
        "SELECT id, action, details, status, timestamp, device_serial FROM activity_log ORDER BY id DESC LIMIT ?1"
    ).map_err(|e| e.to_string())?;
    
    let rows = stmt.query_map(params![limit], |row| {
        Ok(ActivityItem {
            id: row.get(0)?,
            action: row.get(1)?,
            details: row.get(2)?,
            status: row.get(3)?,
            timestamp: row.get(4)?,
            device_serial: row.get(5)?,
        })
    }).map_err(|e| e.to_string())?;
    
    let mut activities = Vec::new();
    for row in rows {
        activities.push(row.map_err(|e| e.to_string())?);
    }
    
    Ok(activities)
}

#[command]
pub async fn clear_activity_log(db: State<'_, HistoryDb>) -> Result<(), String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM activity_log", []).map_err(|e| e.to_string())?;
    Ok(())
}
