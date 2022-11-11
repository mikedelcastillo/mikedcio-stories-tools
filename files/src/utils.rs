use std::path::PathBuf;

use anyhow::Result;
use utils::generate_id;

pub static FILE_ID_LEN: u32 = 16;

pub fn generate_file_id_name(kind: &String, ext: &String) -> (String, PathBuf) {
    let ext = if ext.len() == 0 {
        "unknown".to_string()
    } else {
        ext.to_owned()
    };
    let file_id = generate_id(FILE_ID_LEN);
    let file_name = format!("{}.{}.{}", &file_id, &kind, &ext);
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

pub static IGNORE_FILE_NAMES: [&str; 5] = [".", "..", "cgi-bin", ".ftpquota", ".well-known"];

pub fn filter_ignored(list: Vec<String>) -> Vec<String> {
    let mut output: Vec<String> = vec![];

    for file_name in list {
        if file_name.starts_with(".") {
            continue;
        }
        if IGNORE_FILE_NAMES.contains(&file_name.as_str()) {
            continue;
        }
        output.push(file_name);
    }

    output
}

pub enum MediaType {
    Photo(String),
    Video(String),
    Unknown,
}

pub static MEDIA_EXT_PHOTO: [&str; 5] = ["jpg", "jpeg", "png", "gif", "webp"];
pub static MEDIA_EXT_VIDEO: [&str; 3] = ["mp4", "mov", "avi"];

pub fn ext_to_type(ext: &String) -> MediaType {
    let ext = ext.to_lowercase();

    if MEDIA_EXT_PHOTO.contains(&ext.as_str()) {
        return MediaType::Photo(ext);
    }

    if MEDIA_EXT_VIDEO.contains(&ext.as_str()) {
        return MediaType::Video(ext);
    }

    MediaType::Unknown
}
