use indentation::Indentation;
use position::Row;

pub mod builder;
pub mod indentation;
pub mod position;

pub type NumberedLine = (Row, Line);

#[derive(Clone, Debug)]
pub struct Line {
    pub content: Content,
    pub indentation: Indentation,
}

impl Line {
    pub fn eof() -> Self {
        Self {
            content: Content::Eof,
            indentation: Indentation::none(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Content {
    Eof,
    Value(String),
}

#[derive(Debug, PartialEq)]
pub enum LineKind {
    Selector,
    Declaration,
}

pub fn determine_kind(line: &Line, next_line: &Line) -> LineKind {
    let level = line.indentation.depth;
    let next_level = next_line.indentation.depth;

    if level < next_level {
        LineKind::Selector
    } else {
        LineKind::Declaration
    }
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
            Line::eof(),
        ];

        let expected_types = vec![
            LineKind::Selector,
            LineKind::Selector,
            LineKind::Declaration,
            LineKind::Declaration,
            LineKind::Selector,
            LineKind::Declaration,
            LineKind::Declaration,
            LineKind::Selector,
            LineKind::Declaration,
            LineKind::Declaration,
        ];

        let pass = lines
            .iter()
            .tuple_windows()
            .enumerate()
            .all(|(i, (line, next))| determine_kind(line, next) == expected_types[i]);

        assert!(pass);
    }

    mod helpers {
        use super::*;
        use crate::preprocessor::line::indentation::{Indentation, IndentationKind};

        pub fn line_from(s: &str, depth: usize) -> Line {
            let kind = match depth {
                0 => IndentationKind::None,
                _ => IndentationKind::Space,
            };

            Line {
                content: Content::Value(String::from(s)),
                indentation: Indentation { kind, depth },
            }
        }
    }
}
