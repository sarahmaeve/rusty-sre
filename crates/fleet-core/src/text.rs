use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TextError {
    #[error("requested prefix exceeds input")]
    TooShort,
}

pub fn prefix(input: &str, characters: usize) -> Result<&str, TextError> {
    if input.len() < characters {
        return Err(TextError::TooShort);
    }
    Ok(&input[..characters])
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SizeError {
    #[error("value {0} does not fit in u32")]
    OutOfRange(u64),
}

pub fn narrow_size(value: u64) -> Result<u32, SizeError> {
    Ok(value as u32)
}
