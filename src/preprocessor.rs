use crate::util;
use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct Preprocessor {
    src: Option<PathBuf>,
    out: Option<PathBuf>,
    is_watch_mode: bool,
}

impl Preprocessor {
    pub fn run(&mut self) {
        assert!(self.src.is_some(), "src file is not setted properly");

        if self.out.is_none() {
            self.set_default_out_pathbuf();
        }

        println!("Running the preprocessor");
    }

    pub fn set_src_file(&mut self, file: &Path) {
        util::assert_file_exists(file);
        util::assert_src_file_extension(file);

        self.src = Some(file.to_path_buf());
    }

    pub fn set_out_file(&mut self, file: &Path) {
        self.out = Some(file.to_path_buf());
    }

    pub fn set_to_watch_mode(&mut self) {
        self.is_watch_mode = true;
    }

    fn set_default_out_pathbuf(&mut self) {
        // default out file will have the same filename
        // but with different extension
        let default = self
            .src
            .as_deref()
            .expect("src file is not setted properly")
            .with_extension("css");

        self.set_out_file(&default);
    }
}
