mod path;
use crate::path::get_file_paths;

pub mod download;
pub use crate::download::*;

pub mod utils;
pub use crate::utils::*;

pub fn setup() {
    let file_paths = get_file_paths();

    ensure_dir_exists(&file_paths.root);
    ensure_dir_exists(&file_paths.source);
    ensure_dir_exists(&file_paths.processed);

    println!("{:?}", file_paths)
}
