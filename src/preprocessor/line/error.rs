use super::position::Position;

pub struct Error {
    pub kind: ErrorKind,
    pub pos: Position,
}

pub enum ErrorKind {
    InconsistentIndentation,
    UnexpectedIndentation,
}
