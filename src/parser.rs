#[derive(Parser)]
#[grammar = "grammar/smiley.pest"]
pub struct SmileyParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::{self, consumes_to};

    #[test]
    fn basic_selector_parsing() {
        let cases = [
            "foo",
            "foo.bar",
            "foo.bar.bat",
            "foo#bar.bat",
            ".foo.bar.bat",
            "foo.bar[ baz |= 'string' i ].abc",
            "[foo=bar i][ baz ~= \"bat\" S]",
            ".foo-bar.bat[abc = ghi]",
        ];

        for case in &cases {
            let eoi_pos = case.len();

            pest::parses_to! {
                parser: SmileyParser,
                input: case,
                rule: Rule::selector,
                tokens: [
                    selector(0, eoi_pos, [
                        basic_selector(0, eoi_pos),
                        EOI(eoi_pos, eoi_pos),
                    ]),
                ]
            };
        }
    }

    #[test]
    fn property_parsing() {
        let cases = [
            "max-width: 100px",
            "max-width : 100px",
            "min-width:100VW",
            "font-size: 16pt ! important",
        ];

        for case in &cases {
            let eoi_pos = case.len();

            pest::parses_to! {
                parser: SmileyParser,
                input: case,
                rule: Rule::declaration,
                tokens: [
                    declaration(0, eoi_pos),
                ]
            };
        }
    }
}
