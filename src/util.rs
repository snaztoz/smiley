use log::error;
use std::{
    path::{Path, PathBuf},
    process,
};

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

pub fn create_default_out_file_pathbuf(file: &Path) -> PathBuf {
    let file = file.with_extension("css");
    Path::new(file.file_name().unwrap()).to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_out_file_pathbuf_creation() {
        let src = Path::new("path/to/src.smly");
        let pathbuf = create_default_out_file_pathbuf(&src);

        assert_eq!(pathbuf.as_os_str(), "src.css");
    }
}
