use serde::Serialize;
use anyhow::Error;

#[derive(Debug)]
pub struct CleanerError(pub Error);

impl Serialize for CleanerError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{:#}", self.0))
    }
}

impl From<Error> for CleanerError {
    fn from(err: Error) -> Self {
        CleanerError(err)
    }
}

// Fallback manual mappings for missing contexts defaulting to robust Anywhere
impl From<std::io::Error> for CleanerError {
    fn from(err: std::io::Error) -> Self {
        CleanerError(err.into())
    }
}

impl From<trash::Error> for CleanerError {
    fn from(err: trash::Error) -> Self {
        CleanerError(err.into())
    }
}
