use thiserror::Error;

#[derive(Debug, Error)]
pub enum APIError {
    #[error("Endpoint error: {0}")]
    EndpointError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("File error: {0}")]
    FileError(String),

    #[error("Stream error: {0}")]
    StreamError(String),
}
