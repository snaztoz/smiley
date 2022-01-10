use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct Preprocessor {
    src: Option<PathBuf>,
    out: Option<PathBuf>,
    is_watch_mode: bool,
}

impl Preprocessor {
    pub fn run(&self) {
        assert!(self.src.is_some(), "src file is not set properly");
        assert!(self.out.is_some(), "out file is not set properly");

        println!("Running the preprocessor");
    }

    pub fn set_src_file(&mut self, file: &Path) {
        self.src = Some(file.to_path_buf());
    }

    pub fn set_out_file(&mut self, file: &Path) {
        self.out = Some(file.to_path_buf());
    }

    pub fn set_to_watch_mode(&mut self) {
        self.is_watch_mode = true;
    }
}
