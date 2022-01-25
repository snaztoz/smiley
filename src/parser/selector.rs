#[derive(Parser)]
#[grammar = "grammar/selector.pest"]
pub struct SelectorParser;

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
                parser: SelectorParser,
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
}
