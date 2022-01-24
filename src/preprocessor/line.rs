use super::indentation::IndentationMode;

pub type Row = usize;
pub type Col = usize;

#[derive(Clone, Debug)]
pub struct Line {
    pub row: Row,
    pub content: Content,
    pub indentation_mode: IndentationMode,
}

impl Line {
    pub fn determine_kind(line: &Line, next_line: &Line) -> CssLineKind {
        let level = line.get_indentation_level();
        let next_level = next_line.get_indentation_level();

        if level < next_level {
            CssLineKind::Selector
        } else {
            CssLineKind::Property
        }
    }

    pub fn get_indentation_level(&self) -> Option<usize> {
        self.indentation_mode.map(|(_, level)| level)
    }
}

#[derive(Clone, Debug)]
pub enum Content {
    Eof,
    Value(String),
}

#[derive(Debug, PartialEq)]
pub enum CssLineKind {
    Selector,
    Property,
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn css_type_classification() {
        // Lines:
        //
        // a
        //   b
        //     c
        //     d
        //   e
        //     f
        //   g
        // h
        //   i
        //   j
        //
        let lines = vec![
            helpers::line_from("a", 0),
            helpers::line_from("b", 2),
            helpers::line_from("c", 4),
            helpers::line_from("d", 4),
            helpers::line_from("e", 2),
            helpers::line_from("f", 4),
            helpers::line_from("g", 2),
            helpers::line_from("h", 0),
            helpers::line_from("i", 2),
            helpers::line_from("j", 2),
            helpers::eof(),
        ];

        let expected_types = vec![
            CssLineKind::Selector,
            CssLineKind::Selector,
            CssLineKind::Property,
            CssLineKind::Property,
            CssLineKind::Selector,
            CssLineKind::Property,
            CssLineKind::Property,
            CssLineKind::Selector,
            CssLineKind::Property,
            CssLineKind::Property,
        ];

        let pass = lines
            .iter()
            .tuple_windows()
            .enumerate()
            .all(|(i, (line, next))| Line::determine_kind(line, next) == expected_types[i]);

        assert!(pass);
    }

    mod helpers {
        use super::*;
        use crate::preprocessor::indentation::{Indentation, IndentationLevel};

        pub fn line_from(s: &str, level: IndentationLevel) -> Line {
            let indentation_mode = match level {
                0 => None,
                _ => Some((Indentation::Space, level)),
            };

            Line {
                row: 1, // doesn't matter
                content: Content::Value(String::from(s)),
                indentation_mode,
            }
        }

        pub fn eof() -> Line {
            Line {
                row: usize::MAX,
                content: Content::Eof,
                indentation_mode: None,
            }
        }
    }
}
