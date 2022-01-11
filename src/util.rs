use log::error;
use std::{path::Path, process};

pub fn assert_file_exists(file: &Path) {
    if !file.exists() {
        error!("File does not exist: `{}`", file.display());
        process::exit(1);
    }
}

pub fn assert_src_file_extension(file: &Path) {
    let extension = file.extension();

    if let Some(ext) = extension {
        if ext == "smly" {
            return;
        }
    }

    error!("Invalid extension: Smiley src files should have `.smly` extension");
    process::exit(1);
}
