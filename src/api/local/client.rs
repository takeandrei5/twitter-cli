use tiny_http::{Header, Response, Server, StatusCode};
use url::Url;

use crate::utils::ApplicationError;

const CALLBACK_URL: &str = "/callback";
const LOGIN_SUCCESS_HTML: &str = include_str!("login_success.html");

pub struct LocalClient;

impl LocalClient {
    pub fn start_local_client() -> Result<(String, String), ApplicationError> {
        let port = dotenvy::var("TWITTER_REDIRECT_URL_PORT")?;
        let address = format!("127.0.0.1:{port}");

        let server = Server::http(&address).map_err(|error| {
            ApplicationError::UnexpectedError(std::io::Error::other(error.to_string()))
        })?;

        for request in server.incoming_requests() {
            let callback_url = Url::parse(&format!("http://localhost{}", request.url()))?;

            if callback_url.path() != CALLBACK_URL {
                continue;
            }

            let code = callback_url
                .query_pairs()
                .find(|(key, _)| key == "code")
                .map(|(_, value)| value.into_owned());

            let state = callback_url
                .query_pairs()
                .find(|(key, _)| key == "state")
                .map(|(_, value)| value.into_owned());

            let response = Response::from_string(LOGIN_SUCCESS_HTML)
                .with_header(
                    Header::from_bytes("Content-Type", "text/html; charset=UTF-8").map_err(
                        |_| {
                            ApplicationError::UnexpectedError(std::io::Error::other(
                                "Could not create HTML content type header",
                            ))
                        },
                    )?,
                )
                .with_status_code(StatusCode(200));

            request.respond(response).map_err(|error| {
                ApplicationError::UnexpectedError(std::io::Error::other(error.to_string()))
            })?;

            match (code, state) {
                (Some(code), Some(state)) => return Ok((code, state)),
                _ => continue,
            }
        }

        Err(ApplicationError::UnexpectedError(std::io::Error::other(
            "The OAuth callback did not contain both code and state",
        )))
    }
}
