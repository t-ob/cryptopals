use std::io;

#[derive(Debug, Copy, Clone)]
pub enum DecodeError {
    InvalidCharacter,
    OutOfBounds,
    InvalidLength,
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)
    }
}

impl std::error::Error for DecodeError {}

impl From<DecodeError> for io::Error {
    fn from(err: DecodeError) -> Self {
        io::Error::new(io::ErrorKind::Other, err)
    }
}
