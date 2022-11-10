use std::path::PathBuf;

use anyhow::Result;
use utils::generate_id;

pub static FILE_ID_LEN: u32 = 16;

pub fn generate_file_id_name(ext: &String) -> (String, PathBuf) {
    let ext = if ext.len() == 0 {
        "unknown".to_string()
    } else {
        ext.to_owned()
    };
    let file_id = generate_id(FILE_ID_LEN);
    let file_name = format!("{}.{}", &file_id, &ext);
    let file_name = PathBuf::from(file_name);
    (file_id, file_name)
}

pub fn get_file_ext(filename: &String) -> Result<String> {
    let ext = std::path::Path::new(filename.as_str());
    let ext = ext
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .to_lowercase()
        .to_string();
    Ok(ext)
}

pub fn ensure_dir_exists(path: &PathBuf) {
    if !path.exists() {
        std::fs::create_dir_all(path).expect("Could not create folder");
    }
}
