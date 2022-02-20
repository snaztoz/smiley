use crate::preprocessor::line::position::Position;
use indoc::indoc;

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

impl ErrorKind {
    pub fn get_message(&self) -> String {
        let msg = match self {
            ErrorKind::InconsistentIndentation => indoc! {"
                Inconsistent indentation

                Smiley files should only use either space or tab as
                indentation character, but never both
            "},
            ErrorKind::UnexpectedIndentation => indoc! {"
                Unexpected indentation
            "},
        };

        String::from(msg)
    }
}
