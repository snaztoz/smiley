use super::line::{Col, Line, Row};
use log::debug;
use std::cmp::Ordering;

pub type IndentationMode = Option<(Indentation, IndentationLevel)>;
pub type IndentationLevel = usize;
type Error = (ErrorKind, (Row, Col));

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Indentation {
    Space,
    Tab,
}

impl Indentation {
    pub fn check_mode(line: &str) -> Result<IndentationMode, Col> {
        if line.is_empty() {
            return Ok(None);
        }

        let mut chars = line.chars().enumerate();
        let first_char = chars.next().map(|(_, c)| c);

        let mode = match first_char {
            Some(c) if c == ' ' => Indentation::Space,
            Some(c) if c == '\t' => Indentation::Tab,
            _ => return Ok(None),
        };

        // check trailing chars
        for (i, c) in chars {
            if !c.is_ascii_whitespace() {
                return Ok(Some((mode, i)));
            }

            let is_consistent =
                (mode == Indentation::Space && c == ' ') || (mode == Indentation::Tab && c == '\t');

            if !is_consistent {
                return Err(i);
            }
        }

        Ok(Some((mode, line.len())))
    }
}

#[derive(Default)]
pub struct Validator {
    indentation_type: Option<Indentation>,
    indentation_level_stack: Vec<IndentationLevel>,
}

impl Validator {
    pub fn validate(&mut self, line: &Line) -> Result<(), Error> {
        if line.indentation_mode.is_none() {
            self.set_stack_level_to_zero();
            return Ok(());
        }

        self.validate_indentation_type(line)?;
        self.validate_indentation_level(line)?;

        Ok(())
    }

    fn validate_indentation_type(&mut self, line: &Line) -> Result<(), Error> {
        let (indent, _) = line.indentation_mode.unwrap();

        if self.indentation_type.is_none() {
            self.set_indentation_type(&indent);
            return Ok(());
        }

        if indent != *self.indentation_type.as_ref().unwrap() {
            Err((ErrorKind::InconsistentIndentation, (line.row, 0)))
        } else {
            Ok(())
        }
    }

    fn set_indentation_type(&mut self, indent: &Indentation) {
        if *indent == Indentation::Space {
            debug!("Setting indentation mode to `space`");
        } else {
            debug!("Setting indentation mode to `tab`");
        }

        self.indentation_type = Some(*indent);
    }

    fn validate_indentation_level(&mut self, line: &Line) -> Result<(), Error> {
        // this will prevent non-zero indentation at first
        // non-empty line
        if self.indentation_level_stack.is_empty() {
            return Err((ErrorKind::UnexpectedIndentation, (line.row, 0)));
        }

        let (_, level) = line.indentation_mode.unwrap();
        let &top_level = self.indentation_level_stack.last().unwrap();

        if level < top_level {
            return self
                .pop_stack(level)
                .map_err(|_| (ErrorKind::UnexpectedIndentation, (line.row, level)));
        }

        if level > top_level {
            self.indentation_level_stack.push(level);
        }

        Ok(())
    }

    fn pop_stack(&mut self, new_level: IndentationLevel) -> Result<(), ()> {
        loop {
            let top = self.indentation_level_stack.pop().unwrap();

            match new_level.cmp(&top) {
                Ordering::Equal => return Ok(()),
                Ordering::Greater => break Err(()),
                _ => (),
            }
        }
    }

    fn set_stack_level_to_zero(&mut self) {
        self.indentation_level_stack.clear();
        self.indentation_level_stack.push(0);
    }
}

pub enum ErrorKind {
    InconsistentIndentation,
    UnexpectedIndentation,
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use super::*;

    #[test]
    fn line_indentation_mode_checking() {
        let cases = vec![
            ("",        Ok(None)),
            ("   ",     Ok(Some((Indentation::Space, 3)))),
            ("\t\tfoo", Ok(Some((Indentation::Tab, 2)))),
            ("foo",     Ok(None)),
            (" \tfoo",  Err(1)),
        ];

        for (i, (line, expected)) in cases.iter().enumerate() {
            assert_eq!(
                Indentation::check_mode(line),
                *expected,
                "failed at test case {}",
                i + 1
            );
        }
    }
}
