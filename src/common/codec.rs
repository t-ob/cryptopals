#[derive(Debug, Copy, Clone)]
pub enum DecodeError {
    InvalidCharacter,
    OutOfBounds,
    InvalidLength,
}