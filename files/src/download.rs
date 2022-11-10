

use crate::utils::{generate_file_id_name, get_file_ext};
use anyhow::Result;

pub fn download_from_url(url: String) -> Result<()> {
    let response = reqwest::blocking::get(&url)?;
    let bytes = response.bytes()?;

    let ext = get_file_ext(&url)?;
    let (_file_id, file_name) = generate_file_id_name(&ext);

    println!("file_name: {:?}, size: {}", file_name, bytes.len());

    Ok(())
}
