use anyhow::Result;
use std::fs;
use std::io::Cursor;
use std::path::PathBuf;

use crate::path::get_file_paths;
use crate::utils::{generate_file_id_name, get_file_ext};

pub fn download_from_url(url: String) -> Result<PathBuf> {
    let response = reqwest::blocking::get(&url)?;
    let bytes = response.bytes()?;

    let ext = get_file_ext(&url)?;
    let kind = "source".to_string();
    let (_file_id, file_name) = generate_file_id_name(&kind, &ext);

    let file_paths = get_file_paths();
    let file_path = file_paths.processed.join(file_name);

    let mut file = fs::File::create(&file_path)?;
    let mut bytes = Cursor::new(bytes);
    std::io::copy(&mut bytes, &mut file)?;

    Ok(file_path)
}
