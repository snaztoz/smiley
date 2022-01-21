use super::indentation::IndentationMode;

pub type Row = usize;
pub type Col = usize;

#[derive(Clone, Debug)]
pub struct Line {
    pub row: Row,
    pub content: Content,
    pub indentation_mode: IndentationMode,
}

#[derive(Clone, Debug)]
pub enum Content {
    Eof,
    Value(String),
}
