use super::position::Position;

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub pos: Position,
}

#[derive(Debug)]
pub enum ErrorKind {
    InconsistentIndentation,
    UnexpectedIndentation,
}
