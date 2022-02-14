use super::Pos;

pub struct Error {
    pub kind: ErrorKind,
    pub pos: Pos,
}

pub enum ErrorKind {
    InconsistentIndentation,
    UnexpectedIndentation,
}
