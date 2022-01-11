use crate::util;
use log::{debug, info};
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
            self.set_out_file_to_default();
        }

        info!("Running the preprocessor");
    }

    pub fn set_src_file(&mut self, file: &Path) {
        util::assert_file_exists(file);
        util::assert_src_file_extension(file);

        self.src = Some(file.to_path_buf());

        debug!("The src file is setted to `{}`", file.display());
    }

    pub fn set_out_file(&mut self, file: &Path) {
        self.out = Some(file.to_path_buf());

        debug!("The out file is setted to `{}`", file.display());
    }

    pub fn set_to_watch_mode(&mut self) {
        self.is_watch_mode = true;

        info!("Preprocessor is setted to `watch` mode");
    }

    fn set_out_file_to_default(&mut self) {
        debug!("Setting the out file to default");

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
