use tiny_http::{Response, Server, StatusCode};
use url::Url;

use crate::utils::ApplicationError;

const CALLBACK_URL: &str = "/callback";
const CALLBACK_QUERY_PARAMS: [&str; 2] = ["state", "code"];

pub struct LocalClient;

impl LocalClient {
    pub fn start_local_client() -> Result<(String, String), ApplicationError> {
        let port = dotenvy::var("TWITTER_REDIRECT_URL_PORT")?;
        let server = Server::http(format!("127.0.0.1:{}", port)).unwrap();

        let mut code = None;
        let mut state = None;

        for request in server.incoming_requests() {
            let full_url = request.url();
            let parsed_url = Url::parse(&format!("http://127.0.0.1:{}{}", port, full_url))?;

            if parsed_url.path() != CALLBACK_URL {
                continue;
            }

            parsed_url
                .query_pairs()
                .filter(|(k, _v)| CALLBACK_QUERY_PARAMS.contains(&k.as_ref()))
                .for_each(|(k, v)| {
                    if k.as_ref() == "state" {
                        state = Some(v.into_owned());
                    } else if k.as_ref() == "code" {
                        code = Some(v.into_owned())
                    }
                });

            let _ = request.respond(Response::new_empty(StatusCode(200)));

            if code.is_some() && state.is_some() {
                break;
            }
        }

        Ok((code.unwrap(), state.unwrap()))
    }
}
