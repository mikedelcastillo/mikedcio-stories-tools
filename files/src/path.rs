use std::path::PathBuf;

#[derive(Debug)]
pub struct FilePaths {
    pub root: PathBuf,
    pub source: PathBuf,
    pub processed: PathBuf,
}

pub fn get_file_paths() -> FilePaths {
    let root = PathBuf::from("./data");

    let source = PathBuf::from("source");
    let source = root.join(source);

    let processed = PathBuf::from("processed");
    let processed = root.join(processed);

    FilePaths {
        root,
        source,
        processed,
    }
}
