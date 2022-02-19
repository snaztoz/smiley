use crate::{preprocessor::Preprocessor, util};
use log::debug;
use std::{cell::RefCell, path::Path};

#[derive(Default)]
pub struct Builder {
    preprocessor: RefCell<Preprocessor>,
}

impl Builder {
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

    pub fn build(&self) -> Preprocessor {
        self.preprocessor.take()
    }
}
