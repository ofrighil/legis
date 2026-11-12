#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Core(#[from] legis_core::error::Error),
    #[error("request to {url} failed: {message}")]
    Transport { url: String, message: String },
}

pub type Result<T> = std::result::Result<T, Error>;
