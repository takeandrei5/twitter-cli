use oauth2::{
    HttpClientError, RequestTokenError, StandardErrorResponse, basic::BasicErrorResponseType,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Unexpected error. {0}")]
    UnexpectedError(#[from] std::io::Error),

    #[error("Request token error. {0}")]
    RequestTokenError(
        #[from]
        RequestTokenError<
            HttpClientError<oauth2::reqwest::Error>,
            StandardErrorResponse<BasicErrorResponseType>,
        >,
    ),

    #[error("Environment variables not configured correctly. {0}")]
    EnvironmentVariablesConfigurationError(#[from] dotenvy::Error),

    #[error("Invalid configuration parsed by oauth2 module. {0}")]
    InvalidConfigurationOauth2(#[from] oauth2::url::ParseError),

    #[error("CSRF token mismatch.")]
    CSRFTokenMismatch(),

    #[error("Reqwest error. {0}")]
    ReqwestError(#[from] oauth2::reqwest::Error),

    #[error("X API call request error. {0}")]
    XApiCallError(String),
}
