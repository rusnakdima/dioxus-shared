//! Update downloader.

use std::path::PathBuf;

pub async fn download_update(_url: &str, _destination: PathBuf) -> Result<PathBuf, String> {
    Err("Download not implemented".to_string())
}

pub fn get_temp_download_path(filename: &str) -> PathBuf {
    std::env::temp_dir().join(filename)
}
