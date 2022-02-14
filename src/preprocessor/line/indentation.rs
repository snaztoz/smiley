use super::position::Col;

#[derive(Clone, Debug, PartialEq)]
pub struct Indentation {
    pub kind: IndentationKind,
    pub depth: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IndentationKind {
    None,
    Space,
    Tab,
}

impl Indentation {
    pub fn of_line(line: &str) -> Result<Indentation, Col> {
        if line.is_empty() {
            return Ok(Indentation::none());
        }

        let mut chars = line.chars().enumerate();
        let first_char = chars.next().map(|(_, c)| c);

        let kind = match first_char {
            Some(c) if c == ' ' => IndentationKind::Space,
            Some(c) if c == '\t' => IndentationKind::Tab,
            _ => return Ok(Indentation::none()),
        };

        // check trailing chars
        for (i, c) in chars {
            if !c.is_ascii_whitespace() {
                return Ok(Indentation { kind, depth: i });
            }

            let is_consistent = (kind == IndentationKind::Space && c == ' ')
                || (kind == IndentationKind::Tab && c == '\t');

            if !is_consistent {
                return Err(i);
            }
        }

        Ok(Indentation {
            kind,
            depth: line.len(),
        })
    }

    pub fn none() -> Self {
        Self {
            kind: IndentationKind::None,
            depth: 0,
        }
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use super::*;

    #[test]
    fn line_indentation_mode_checking() {
        let cases = vec![
            ("",        Ok(Indentation::none())),
            ("   ",     Ok(Indentation {kind: IndentationKind::Space, depth: 3})),
            ("\t\tfoo", Ok(Indentation {kind: IndentationKind::Tab, depth: 2})),
            ("foo",     Ok(Indentation::none())),
            (" \tfoo",  Err(1)),
        ];

        for (i, (line, expected)) in cases.iter().enumerate() {
            assert_eq!(
                Indentation::of_line(line),
                *expected,
                "failed at test case {}",
                i + 1
            );
        }
    }
}
