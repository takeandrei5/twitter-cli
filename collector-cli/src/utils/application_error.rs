use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Could not parse message data as UTF-8: {0}.")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("Could not parse message data as JSON: {0}.")]
    SerializeDeserialize(#[from] serde_json::Error),

    #[error("Invalid input: {0}.")]
    BadRequest(String),

    #[error("Resource with identifier {0} was not found.")]
    NotFound(String),

    #[error("Unauthorized access.")]
    Unauthorized,

    #[error("An internal server error occurred: {0}.")]
    InternalServer(String),

    // #[error("Failed to send a request to an external service: {0}.")]
    // Request(#[from] reqwest::Error),
    #[error("Unexpected error. {0}.")]
    UnexpectedError(#[from] std::io::Error),
}