use std::path::PathBuf;

pub fn get_app_data_dir() -> Result<PathBuf, String> {
    dirs::data_local_dir()
        .map(|p| p.join("OpenMonkey"))
        .ok_or_else(|| "Failed to get data directory".to_string())
}
