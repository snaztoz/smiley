use crate::{preprocessor::Preprocessor, util};
use log::{debug, info};
use std::{cell::RefCell, path::Path};

#[derive(Default)]
pub struct PreprocessorBuilder {
    preprocessor: RefCell<Preprocessor>,
}

impl PreprocessorBuilder {
    pub fn set_src_file(&self, file: &Path) -> &Self {
        util::assert_file_exists(file);
        util::assert_src_file_extension(file);

        debug!("Setting src file to `{}`", file.display());
        self.preprocessor.borrow_mut().src = Some(file.to_path_buf());

        self
    }

    pub fn set_out_file(&self, file: Option<&Path>) -> &Self {
        let file = match file {
            Some(f) => f.to_path_buf(),
            None => {
                debug!("No out file specified. Use default value");
                util::create_default_out_file_pathbuf(
                    self.preprocessor
                        .borrow_mut()
                        .src
                        .as_deref()
                        .expect("src file is not setted properly"),
                )
            }
        };

        debug!("Setting out file to `{}`", file.display());
        self.preprocessor.borrow_mut().out = Some(file);

        self
    }

    pub fn set_watch_mode(&self, watch_mode: bool) -> &Self {
        info!("Setting preprocessor to `watch` mode");
        self.preprocessor.borrow_mut().is_watch_mode = watch_mode;

        self
    }

    pub fn build(&self) -> Preprocessor {
        self.preprocessor.take()
    }
}
