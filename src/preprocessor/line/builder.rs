use super::{
    error::{Error as LineError, ErrorKind as LineErrorKind},
    Content as LineContent, Line, Row,
};
use crate::preprocessor::indentation::Indentation;

#[derive(Default)]
pub struct Builder {
    row_count: Row,
}

impl Builder {
    pub fn build_line_from(&mut self, raw_line: &str) -> Result<Option<Line>, LineError> {
        self.row_count += 1;

        if raw_line.trim().is_empty() {
            return Ok(None);
        }

        let indent_mode = Indentation::mode_of(raw_line);
        let line = indent_mode
            .map(|mode| Line {
                // remove indentations, and put the information
                // inside indentation_mode instead
                content: LineContent::Value(raw_line.trim().to_string()),
                indentation_mode: mode,
                row: self.row_count,
            })
            .map_err(|col| LineError {
                kind: LineErrorKind::InconsistentIndentation,
                pos: (self.row_count, col),
            })?;

        Ok(Some(line))
    }
}