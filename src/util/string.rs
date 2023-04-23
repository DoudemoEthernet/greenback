use std::{cell::RefCell, error::Error, fmt::Display};

#[derive(Debug)]
pub struct TryFromStringError {
    pub raw: String,
    pub kind: TryFromStringErrorKind,
}

impl Display for TryFromStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to parse string original:{}", self.raw)
    }
}

impl Error for TryFromStringError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.kind {
            TryFromStringErrorKind::Uuid(e) => Some(e),
        }
    }
}

#[derive(Debug)]
pub enum TryFromStringErrorKind {
    Uuid(uuid::Error),
}
