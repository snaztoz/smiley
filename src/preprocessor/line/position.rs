pub type Row = usize;
pub type Col = usize;

pub struct Position {
    pub row: Row,
    pub col: Col,
}

impl Position {
    pub fn at(row: Row, col: Col) -> Self {
        Self { row, col }
    }
}
