use super::indentation::IndentationMode;

#[derive(Clone, Debug)]
pub struct Line {
    pub row: usize,
    pub content: Content,
    pub indentation_mode: IndentationMode,
}

#[derive(Clone, Debug)]
pub enum Content {
    Eof,
    Value(String),
}
