
#[derive(Debug, thiserror::Error)]
pub enum Error {
    
    #[error("JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("IO: {0}")]
    IO(#[from] std::io::Error),

    #[error("Generic: {0}")]
    Config(&'static str)
}

impl From<&'static str> for Error{
    fn from(value: &'static str) -> Self {
        Self::Config(value)
    }
}