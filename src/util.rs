use log::error;
use std::{
    fs,
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

pub fn read_file_with_empty_line_appended(file: &Path) -> String {
    let mut content = fs::read_to_string(file).unwrap().trim_end().to_string();
    content.push_str("\n\n");

    content
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::fixture::{FileWriteStr, NamedTempFile};

    #[test]
    fn default_out_file_pathbuf_creation() {
        let src = Path::new("path/to/src.smly");
        let pathbuf = create_default_out_file_pathbuf(&src);

        assert_eq!(pathbuf.as_os_str(), "src.css");
    }

    #[test]
    fn read_file_with_empty_line_appended_checking() {
        let cases = [
            (" foo", " foo\n\n"),
            ("bar\n", "bar\n\n"),
            ("baz\n\n\n", "baz\n\n"),
        ];

        for (i, (content, expected)) in cases.iter().enumerate() {
            let file = NamedTempFile::new("foo").unwrap();
            file.write_str(content).unwrap();

            assert_eq!(
                &read_file_with_empty_line_appended(file.path()),
                expected,
                "failed at test case {}",
                i + 1
            );
        }
    }
}
