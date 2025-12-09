use tauri::command;
use std::path::Path;
use std::fs::File;
use sha2::{Sha256, Digest};

#[derive(serde::Serialize)]
pub struct FileMetadata {
    pub name: String,
    pub size: u64,
    pub hash: String,
    pub rom_type: String,
}

#[command]
pub async fn parse_rom_file(file_path: String) -> Result<FileMetadata, String> {
    let path = Path::new(&file_path);
    if !path.exists() {
        return Err("File does not exist".to_string());
    }

    let metadata = std::fs::metadata(path).map_err(|e| e.to_string())?;
    let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
    
    // Determine type
    let extension = path.extension().unwrap_or_default().to_string_lossy().to_lowercase();
    let rom_type = match extension.as_str() {
        "zip" => "Recovery ZIP",
        "img" => "Fastboot Image",
        "bin" => "Payload Binary",
        _ => "Unknown",
    }.to_string();

    // Calculate SHA256
    let hash = calculate_sha256(path)?;

    Ok(FileMetadata {
        name,
        size: metadata.len(),
        hash,
        rom_type,
    })
}

pub fn calculate_sha256(path: &Path) -> Result<String, String> {
    let mut file = File::open(path).map_err(|e| e.to_string())?;
    let mut hasher = Sha256::new();
    std::io::copy(&mut file, &mut hasher).map_err(|e| e.to_string())?;
    let hash = hasher.finalize();
    Ok(format!("{:x}", hash))
}

pub fn calculate_md5(path: &Path) -> Result<String, String> {
    let mut file = File::open(path).map_err(|e| e.to_string())?;
    let mut hasher = md5::Context::new();
    std::io::copy(&mut file, &mut hasher).map_err(|e| e.to_string())?;
    let hash = hasher.compute();
    Ok(format!("{:x}", hash))
}

#[command]
pub async fn verify_file_checksum(file_path: String, expected_hash: String) -> Result<bool, String> {
    let path = Path::new(&file_path);
    // Try SHA256 first
    if let Ok(calculated) = calculate_sha256(path) {
        if calculated.eq_ignore_ascii_case(&expected_hash) {
            return Ok(true);
        }
    }
    // Try MD5
    if let Ok(calculated) = calculate_md5(path) {
        if calculated.eq_ignore_ascii_case(&expected_hash) {
            return Ok(true);
        }
    }
    Ok(false)
}

#[command]
pub async fn verify_file_md5(file_path: String, expected_hash: String) -> Result<bool, String> {
    let path = Path::new(&file_path);
    let calculated = calculate_md5(path)?;
    Ok(calculated.eq_ignore_ascii_case(&expected_hash))
}

#[command]
pub async fn extract_zip_file(file_path: String, output_dir: String) -> Result<String, String> {
    let path = Path::new(&file_path);
    let out_path = Path::new(&output_dir);
    
    let file = File::open(path).map_err(|e| e.to_string())?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        let outpath = match file.enclosed_name() {
            Some(path) => out_path.join(path),
            None => continue,
        };

        if (*file.name()).ends_with('/') {
            std::fs::create_dir_all(&outpath).map_err(|e| e.to_string())?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p).map_err(|e| e.to_string())?;
                }
            }
            let mut outfile = File::create(&outpath).map_err(|e| e.to_string())?;
            std::io::copy(&mut file, &mut outfile).map_err(|e| e.to_string())?;
        }
    }
    Ok("Extraction complete".to_string())
}
