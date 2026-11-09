#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("no API key configured")]
    MissingApiKey,
    #[error("HTTP {status} from {url}")]
    Http { status: u16, url: String },
    #[error("could not decode response from {url}")]
    Decode {
        url: String,
        source: serde_json::Error,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
