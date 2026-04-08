use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum CleanerError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Trash error: {0}")]
    Trash(#[from] trash::Error),
    #[error("Database error: {0}")]
    Database(String),
}

// Needed so Tauri can return CleanerError natively in invoke calls
impl Serialize for CleanerError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
