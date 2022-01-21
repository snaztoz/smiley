use super::line::{Col, Line, Row};
use log::debug;

pub type IndentationMode = Option<(Indentation, IndentationLevel)>;
pub type IndentationLevel = usize;

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
pub struct Checker {
    used_type: Option<Indentation>,
}

impl Checker {
    pub fn validate(&mut self, line: &Line) -> Result<(), (Row, Col)> {
        if line.indentation_mode.is_none() {
            return Ok(());
        }

        let (indent, _) = line.indentation_mode.unwrap();

        if self.handle_indentation(&indent).is_err() {
            Err((line.row, 0))
        } else {
            Ok(())
        }
    }

    fn handle_indentation(&mut self, indent: &Indentation) -> Result<(), ()> {
        match self.used_type {
            Some(_) => self.validate_indentation_type(indent),
            None => {
                self.set_indentation_mode(indent);
                Ok(())
            }
        }
    }

    fn validate_indentation_type(&self, indent: &Indentation) -> Result<(), ()> {
        if indent != self.used_type.as_ref().unwrap() {
            Err(())
        } else {
            Ok(())
        }
    }

    fn set_indentation_mode(&mut self, indent: &Indentation) {
        if *indent == Indentation::Space {
            debug!("Setting indentation mode to `space`");
        } else {
            debug!("Setting indentation mode to `tab`");
        }

        self.used_type = Some(*indent);
    }
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
