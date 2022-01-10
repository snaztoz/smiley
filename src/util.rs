use std::{path::Path, process};

pub fn assert_file_exists(file: &Path) {
    if !file.exists() {
        eprintln!("file does not exists: {}", file.display());
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

    eprintln!("invalid extension: Smiley src files should have '.smly' extension");
    process::exit(1);
}
