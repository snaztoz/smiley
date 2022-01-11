#[derive(Parser)]
#[grammar = "grammar/smiley.pest"]
pub struct SmileyParser;

#[derive(Debug, PartialEq)]
pub enum Indentation {
    None,
    Space,
    Tab,
}

impl Indentation {
    pub fn check_mode(line: &str) -> Result<Indentation, ()> {
        if line.is_empty() {
            return Ok(Indentation::None);
        }

        let mut chars = line.chars();
        let first_char = chars.next();

        let mode = match first_char {
            Some(c) if c == ' ' => Indentation::Space,
            Some(c) if c == '\t' => Indentation::Tab,
            _ => return Ok(Indentation::None),
        };

        // check trailing chars
        for c in chars {
            if !c.is_ascii_whitespace() {
                return Ok(mode);
            }

            let is_consistent =
                (mode == Indentation::Space && c == ' ') || (mode == Indentation::Tab && c == '\t');

            if !is_consistent {
                return Err(());
            }
        }

        Ok(mode)
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use super::*;

    #[test]
    fn line_indentation_mode_checking() {
        let cases: Vec<(&str, Result<Indentation, ()>)> = vec![
            ("",        Ok(Indentation::None)),
            ("   ",     Ok(Indentation::Space)),
            ("\t\tfoo", Ok(Indentation::Tab)),
            ("foo",     Ok(Indentation::None)),
            (" \tfoo",  Err(())),
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
