type IndentationErrorPos = usize;
pub type IndentationMode = Option<(Indentation, IndentationLevel)>;
pub type IndentationLevel = usize;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Indentation {
    Space,
    Tab,
}

impl Indentation {
    pub fn check_mode(line: &str) -> Result<IndentationMode, IndentationErrorPos> {
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
