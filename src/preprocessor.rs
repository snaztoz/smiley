use crate::{
    error,
    parser::{Indentation, IndentationMode},
    util,
};
use log::{debug, info};
use std::{
    cell::RefCell,
    fs,
    path::{Path, PathBuf},
    process,
};

#[derive(Default)]
pub struct Preprocessor {
    src: Option<PathBuf>,
    out: Option<PathBuf>,
    is_watch_mode: bool,

    indent_type: Option<Indentation>,
    current_row: usize,
}

impl Preprocessor {
    pub fn run(&mut self) {
        assert!(self.src.is_some(), "src file is not setted properly");
        assert!(self.out.is_some(), "out file is not setted properly");

        info!("Running the preprocessor");

        let file = fs::read_to_string(self.src.as_ref().unwrap()).unwrap();

        for line in file.lines() {
            self.current_row += 1;

            // skip if the line does not contain any character
            // other than whitespaces
            if line.trim().is_empty() {
                continue;
            }

            if let Some(indent) = self.get_line_indentation_mode(line) {
                self.handle_line_indentation(indent);
            }
        }
    }

    fn get_line_indentation_mode(&self, line: &str) -> IndentationMode {
        match Indentation::check_mode(line) {
            Ok(m) => m,
            Err(col) => {
                error::report(
                    self.src.as_deref().unwrap(),
                    self.current_row,
                    col,
                    "Inconsistent indentation: Smiley src files should only use either \
                    space\n\tor tab as indentation character, but not both",
                );
                process::exit(1);
            }
        }
    }

    fn handle_line_indentation(&mut self, indent: Indentation) {
        match self.indent_type {
            Some(_) => self.validate_indentation_mode(indent),
            None => self.set_used_indentation_type(indent),
        }
    }

    fn set_used_indentation_type(&mut self, indent: Indentation) {
        if indent == Indentation::Space {
            debug!("Setting indentation mode to `space`");
        } else {
            debug!("Setting indentation mode to `tab`");
        }

        self.indent_type = Some(indent);
    }

    fn validate_indentation_mode(&self, indent: Indentation) {
        if indent != *self.indent_type.as_ref().unwrap() {
            error::report(
                self.src.as_deref().unwrap(),
                self.current_row,
                0,
                "Inconsistent indentation: Smiley src files should only use either \
                space\n\tor tab as indentation character, but not both",
            );

            process::exit(1);
        }
    }
}

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
