use std::{path::Path, process};

pub fn assert_file_exists(file: &Path) {
    if !file.exists() {
        eprintln!("file does not exists: {}", file.display());
        process::exit(1);
    }
}
