use super::{
    error::{Error as LineError, ErrorKind as LineErrorKind},
    indentation::{Indentation, IndentationKind},
    position::{Position, Row},
    Content as LineContent, Line, NumberedLine,
};
use log::debug;

#[derive(Default)]
pub struct Builder {
    row_count: Row,
    indentation_handler: IndentationHandler,
}

impl Builder {
    pub fn build_line_from(&mut self, raw_line: &str) -> Result<Option<NumberedLine>, LineError> {
        self.row_count += 1;

        if raw_line.trim().is_empty() {
            return Ok(None);
        }

        let indent = Indentation::of_line(raw_line).map_err(|col| LineError {
            kind: LineErrorKind::InconsistentIndentation,
            pos: Position::at(self.row_count, col),
        })?;

        self.indentation_handler
            .handle(indent)
            .map_err(|kind| LineError {
                kind,
                pos: Position::at(self.row_count, 0),
            })?;

        Ok(Some((
            self.row_count,
            Line {
                // remove indentations, and put the information
                // inside indentation_mode instead
                content: LineContent::Value(raw_line.trim().to_string()),
                indentation: indent,
            },
        )))
    }
}

#[derive(Default)]
struct IndentationHandler {
    used_kind: Option<IndentationKind>,
    stack: Vec<usize>,
}

impl IndentationHandler {
    fn handle(&mut self, indent: Indentation) -> Result<(), LineErrorKind> {
        self.handle_kind(indent.kind)?;
        self.handle_depth(indent.depth)?;

        Ok(())
    }

    fn handle_kind(&mut self, indent_kind: IndentationKind) -> Result<(), LineErrorKind> {
        if self.used_kind.is_none() && indent_kind != IndentationKind::None {
            self.set_used_indentation_kind(indent_kind);
            return Ok(());
        }

        if indent_kind != IndentationKind::None && indent_kind != self.used_kind.unwrap() {
            return Err(LineErrorKind::InconsistentIndentation);
        }

        Ok(())
    }

    fn handle_depth(&mut self, depth: usize) -> Result<(), LineErrorKind> {
        if self.stack.is_empty() {
            // this will prevent the first non-empty line to
            // have indentation(s)
            if depth == 0 {
                self.stack.push(0);
            } else {
                return Err(LineErrorKind::UnexpectedIndentation);
            }
        }

        let &deepest = self.stack.last().unwrap();
        if depth > deepest {
            self.stack.push(depth);
            return Ok(());
        }

        loop {
            // it won't be less than zero, so it's fine to unwrap
            let &deepest = self.stack.last().unwrap();

            if depth < deepest {
                self.stack.pop().unwrap();
                continue;
            }

            if depth > deepest {
                break Err(LineErrorKind::UnexpectedIndentation);
            }

            break Ok(());
        }
    }

    fn set_used_indentation_kind(&mut self, indent_kind: IndentationKind) {
        assert!(indent_kind != IndentationKind::None);

        if indent_kind == IndentationKind::Space {
            debug!("Setting used indentation type to space");
        } else {
            debug!("Setting used indentation type to tab");
        }

        self.used_kind = Some(indent_kind);
    }
}

#[cfg(test)]
mod tests {
    use super::Builder as LineBuilder;
    use indoc::indoc;

    #[test]
    fn build_normal_lines() {
        let mut builder = LineBuilder::default();
        let src = indoc! {"
            foo
                foo
                foo
                    foo
                foo
                foo
        "};

        for line in src.lines() {
            assert!(builder.build_line_from(line).is_ok());
        }
    }

    #[test]
    fn build_with_inconsistent_indentation() {
        let mut builder = LineBuilder::default();
        let src = indoc! {"
            foo
                foo
            \tfoo
        "};

        let mut lines = src.lines();

        // consume until before last
        builder.build_line_from(lines.next().unwrap()).unwrap();
        builder.build_line_from(lines.next().unwrap()).unwrap();

        assert!(builder.build_line_from(lines.next().unwrap()).is_err());
    }

    #[test]
    fn build_with_unexpected_indentation_1() {
        let mut builder = LineBuilder::default();
        let src = indoc! {"
            \tfoo
        "};

        assert!(builder.build_line_from(src).is_err());
    }

    #[test]
    fn build_with_unexpected_indentation_2() {
        let mut builder = LineBuilder::default();
        let src = indoc! {"
            foo
                foo
                    foo
                foo
              foo
        "};

        let mut lines = src.lines();

        // consume until before last
        builder.build_line_from(lines.next().unwrap()).unwrap();
        builder.build_line_from(lines.next().unwrap()).unwrap();
        builder.build_line_from(lines.next().unwrap()).unwrap();
        builder.build_line_from(lines.next().unwrap()).unwrap();

        assert!(builder.build_line_from(lines.next().unwrap()).is_err());
    }
}
