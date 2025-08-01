use crate::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

#[cfg(target_os = "windows")]
pub fn get_default_log_path() -> Result<PathBuf> {
    let userprofile = std::env::var("USERPROFILE")
        .map_err(|_| eyre!("Could not find USERPROFILE environment variable"))?;
    let log_path = PathBuf::from(userprofile)
        .join("AppData")
        .join("LocalLow")
        .join("Wizards Of The Coast")
        .join("MTGA")
        .join("Logs")
        .join("Logs");

    if !log_path.exists() {
        return Err(eyre!(
            "MTG Arena log directory not found at: {:?}",
            log_path
        ));
    }

    Ok(log_path)
}

#[cfg(target_os = "macos")]
pub fn get_default_log_path() -> Result<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| eyre!("Could not find home directory"))?;
    let log_path = home
        .join("Library")
        .join("Application Support")
        .join("com.wizards.mtga")
        .join("Logs")
        .join("Logs");

    if !log_path.exists() {
        return Err(eyre!(
            "MTG Arena log directory not found at: {:?}",
            log_path
        ));
    }

    Ok(log_path)
}

#[cfg(target_os = "windows")]
pub fn get_player_log_path() -> Result<PathBuf> {
    let userprofile = std::env::var("USERPROFILE")
        .map_err(|_| eyre!("Could not find USERPROFILE environment variable"))?;
    let log_path = PathBuf::from(userprofile)
        .join("AppData")
        .join("LocalLow")
        .join("Wizards Of The Coast")
        .join("MTGA")
        .join("Player.log");

    if !log_path.exists() {
        return Err(eyre!("MTG Arena Player.log not found at: {:?}", log_path));
    }

    Ok(log_path)
}

#[cfg(target_os = "macos")]
pub fn get_player_log_path() -> Result<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| eyre!("Could not find home directory"))?;
    let log_path = home
        .join("Library")
        .join("Logs")
        .join("Wizards Of The Coast")
        .join("MTGA")
        .join("Player.log");

    if !log_path.exists() {
        return Err(eyre!("MTG Arena Player.log not found at: {:?}", log_path));
    }

    Ok(log_path)
}

pub fn find_newest_log_file(dir: &Path) -> Result<PathBuf> {
    let mut newest_file = None;
    let mut newest_time = std::time::SystemTime::UNIX_EPOCH;
    let mut log_files_found = 0;

    if !dir.exists() {
        return Err(eyre!("Directory does not exist: {:?}", dir));
    }

    if !dir.is_dir() {
        return Err(eyre!("Path is not a directory: {:?}", dir));
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("log") {
            log_files_found += 1;
            let metadata = entry.metadata()?;
            if let Ok(modified) = metadata.modified() {
                if modified > newest_time {
                    newest_time = modified;
                    newest_file = Some(path);
                }
            }
        }
    }

    newest_file.ok_or_else(|| {
        if log_files_found == 0 {
            eyre!("No log files found in directory: {:?}", dir)
        } else {
            eyre!(
                "Found {} log files but couldn't determine newest",
                log_files_found
            )
        }
    })
}
