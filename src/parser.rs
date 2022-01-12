#[derive(Parser)]
#[grammar = "grammar/smiley.pest"]
pub struct SmileyParser;

type IndentationErrorPos = usize;
pub type IndentationMode = Option<Indentation>;

#[derive(Debug, PartialEq)]
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
                return Ok(Some(mode));
            }

            let is_consistent =
                (mode == Indentation::Space && c == ' ') || (mode == Indentation::Tab && c == '\t');

            if !is_consistent {
                return Err(i);
            }
        }

        Ok(Some(mode))
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
            ("   ",     Ok(Some(Indentation::Space))),
            ("\t\tfoo", Ok(Some(Indentation::Tab))),
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
