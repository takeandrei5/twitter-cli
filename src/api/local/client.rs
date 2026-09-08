use std::env;
use std::io::Cursor;
use std::time::{Duration, Instant};

use tiny_http::{Header, Response, Server, StatusCode};
use url::Url;

use crate::utils::ApplicationError;

const CALLBACK_URL: &str = "/callback";
const LOGIN_SUCCESS_HTML: &str = include_str!("login_success.html");

pub struct LocalClient;

impl LocalClient {
    fn try_retrieve_code(callback_url: &Url) -> Option<String> {
        callback_url
            .query_pairs()
            .find(|(key, _)| key == "code")
            .map(|(_, value)| value.into_owned())
    }

    fn try_retrieve_state(callback_url: &Url) -> Option<String> {
        callback_url
            .query_pairs()
            .find(|(key, _)| key == "state")
            .map(|(_, value)| value.into_owned())
    }

    fn create_response() -> Response<Cursor<Vec<u8>>> {
        Response::from_string(LOGIN_SUCCESS_HTML)
            .with_header(
                Header::from_bytes("Content-Type", "text/html; charset=UTF-8")
                    .expect("html response should not fail"),
            )
            .with_status_code(StatusCode(200))
    }

    pub fn start_local_client() -> Result<(String, String), ApplicationError> {
        let port = env::var("TWITTER_REDIRECT_URL_PORT")
            .expect("Expected environment variable 'TWITTER_REDIRECT_URL_PORT' to be set");

        let max_wait_time: u64 = env::var("MAX_WAIT_TIME")
            .expect("Expected environment variable 'MAX_WAIT_TIME' to be set")
            .parse()
            .expect("Expected a numerical wait time");
        let address = format!("127.0.0.1:{port}");

        let server = Server::http(&address).map_err(|error| {
            ApplicationError::UnexpectedError(std::io::Error::other(error.to_string()))
        })?;

        let timeout = Duration::from_secs(max_wait_time);
        let started_at = Instant::now();

        loop {
            let remaining = timeout.saturating_sub(started_at.elapsed());

            if let Some(request) = server.recv_timeout(remaining)? {
                let callback_url = Url::parse(&format!("http://localhost{}", request.url()))?;

                if callback_url.path() != CALLBACK_URL {
                    continue;
                }

                let code = Self::try_retrieve_code(&callback_url);
                let state = Self::try_retrieve_state(&callback_url);

                let response = Self::create_response();

                request.respond(response).map_err(|error| {
                    ApplicationError::UnexpectedError(std::io::Error::other(error.to_string()))
                })?;

                match (code, state) {
                    (Some(code), Some(state)) => return Ok((code, state)),
                    _ => continue,
                }
            } else {
                break;
            }
        }

        Err(ApplicationError::UnexpectedError(std::io::Error::other(
            "The OAuth callback did not contain both code and state",
        )))
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::thread;
    use std::time::Duration;

    use rstest::rstest;
    use serial_test::serial;
    use tiny_http::StatusCode;
    use url::Url;

    use crate::api::LocalClient;

    fn send_request(port: u16, request: String) {
        let mut stream = TcpStream::connect(format!("127.0.0.1:{port}"))
            .expect("should connect to local client");

        stream
            .write_all(request.as_bytes())
            .expect("request should be sent");

        // Read until the server closes this connection.
        let mut response = String::new();
        stream
            .read_to_string(&mut response)
            .expect("response should be readable");
    }

    fn cleanup() {
        unsafe {
            std::env::remove_var("TWITTER_REDIRECT_URL_PORT");
            std::env::remove_var("MAX_WAIT_TIME");
        }
    }

    #[test]
    fn create_response_should_return_success_html_response() {
        // Arrange
        let response = LocalClient::create_response();

        // Act
        let status_code = response.status_code();
        let content_type = response
            .headers()
            .iter()
            .find(|header| header.field.equiv("Content-Type"))
            .map(|header| header.value.as_str().to_owned());
        let mut body = String::new();
        response
            .into_reader()
            .read_to_string(&mut body)
            .expect("response body should be readable");

        // Assert
        assert_eq!(status_code, StatusCode(200));
        assert_eq!(content_type.as_deref(), Some("text/html; charset=UTF-8"));
        assert_eq!(body, super::LOGIN_SUCCESS_HTML);
    }

    #[test]
    #[should_panic(
        expected = "Expected environment variable 'TWITTER_REDIRECT_URL_PORT' to be set"
    )]
    fn start_local_client_should_panic_when_twitter_redirect_url_environment_variable_is_missing() {
        // Arrange
        unsafe {
            std::env::remove_var("TWITTER_REDIRECT_URL_PORT");
            std::env::remove_var("MAX_WAIT_TIME");
        }

        // Act
        let _ = LocalClient::start_local_client();

        // Assert

        cleanup();
    }

    #[test]
    #[should_panic(expected = "Expected environment variable 'MAX_WAIT_TIME' to be set")]
    fn start_local_client_should_panic_when_max_wait_time_environment_variable_is_missing() {
        // Arange
        let port = 12_345;

        unsafe {
            std::env::set_var("TWITTER_REDIRECT_URL_PORT", port.to_string());
        }

        // Act
        let _ = LocalClient::start_local_client();

        // Assert

        cleanup();
    }

    #[test]
    #[should_panic(expected = "Expected a numerical wait time")]
    fn start_local_client_should_panic_when_max_wait_time_environment_variable_is_not_numerical() {
        // Arange
        let port = 12_345;

        unsafe {
            std::env::set_var("TWITTER_REDIRECT_URL_PORT", port.to_string());
            std::env::set_var("MAX_WAIT_TIME", "notanumber");
        }

        // Act
        let _ = LocalClient::start_local_client();

        // Assert

        cleanup();
    }

    #[serial]
    #[test]
    fn start_local_client_should_return_success_result() {
        // Arrange
        let expected_result = (String::from("test-code"), String::from("test-state"));
        let port = 38_765;

        unsafe {
            std::env::set_var("TWITTER_REDIRECT_URL_PORT", port.to_string());
            std::env::set_var("MAX_WAIT_TIME", "10");
        }

        // Act
        let client_thread = thread::spawn(LocalClient::start_local_client);
        thread::sleep(Duration::from_millis(250));

        send_request(
            port,
            "GET /callback?code=test-code&state=test-state HTTP/1.1\r\n\
              Host: localhost\r\n\
              Connection: close\r\n\
              \r\n"
                .to_string(),
        );

        let result = client_thread
            .join()
            .expect("local client thread should not panic")
            .expect("result should be success");

        // Assert
        assert_eq!(result, expected_result);

        cleanup();
    }

    #[serial]
    #[test]
    fn start_local_client_should_ignore_partial_requests_and_return_error_when_timeout_expires() {
        // Arrange
        let port = 38_766;

        unsafe {
            std::env::set_var("TWITTER_REDIRECT_URL_PORT", port.to_string());
            std::env::set_var("MAX_WAIT_TIME", "5");
        }

        // Act
        let client_thread = thread::spawn(LocalClient::start_local_client);
        thread::sleep(Duration::from_millis(250));

        send_request(
            port,
            "GET /callback?state=partial-state HTTP/1.1\r\n\
              Host: localhost\r\n\
              Connection: close\r\n\
              \r\n"
                .to_string(),
        );

        send_request(
            port,
            "GET /callback?code=partial-code HTTP/1.1\r\n\
              Host: localhost\r\n\
              Connection: close\r\n\
              \r\n"
                .to_string(),
        );

        let result = client_thread
            .join()
            .expect("local client thread should not panic");

        // Assert
        assert_eq!(
            result.unwrap_err().to_string(),
            "Unexpected error. The OAuth callback did not contain both code and state"
        );

        cleanup();
    }

    #[serial]
    #[test]
    fn start_local_client_should_ignore_partial_requests_and_return_success_result_when_correct_request_is_sent()
     {
        // Arrange
        let expected_result = (String::from("test-code"), String::from("test-state"));
        let port = 38_767;

        unsafe {
            std::env::set_var("TWITTER_REDIRECT_URL_PORT", port.to_string());
            std::env::set_var("MAX_WAIT_TIME", "5");
        }

        // Act
        let client_thread = thread::spawn(LocalClient::start_local_client);
        thread::sleep(Duration::from_millis(250));

        send_request(
            port,
            "GET /callback?state=partial-state HTTP/1.1\r\n\
              Host: localhost\r\n\
              Connection: close\r\n\
              \r\n"
                .to_string(),
        );

        send_request(
            port,
            "GET /callback?code=partial-code HTTP/1.1\r\n\
              Host: localhost\r\n\
              Connection: close\r\n\
              \r\n"
                .to_string(),
        );

        send_request(
            port,
            "GET /callback?code=test-code&state=test-state HTTP/1.1\r\n\
              Host: localhost\r\n\
              Connection: close\r\n\
              \r\n"
                .to_string(),
        );

        let result = client_thread
            .join()
            .expect("local client thread should not panic")
            .expect("result should be success");

        // Assert
        assert_eq!(result, expected_result);

        cleanup();
    }

    #[rstest]
    #[case("http://127.0.0.1/missing", None)]
    #[case("http://127.0.0.1/hello?state=test", Some(String::from("test")))]
    fn try_retrieve_state_should_return_correct_result(
        #[case] url: &str,
        #[case] state: Option<String>,
    ) {
        // Arrange
        let url = Url::parse(url).expect("url should be valid");

        // Act
        let result = LocalClient::try_retrieve_state(&url);

        // Assert
        assert_eq!(result, state)
    }

    #[rstest]
    #[case("http://127.0.0.1/missing", None)]
    #[case("http://127.0.0.1/hello?code=test", Some(String::from("test")))]
    fn try_retrieve_code_should_return_correct_result(
        #[case] url: &str,
        #[case] code: Option<String>,
    ) {
        // Arrange
        let url = Url::parse(url).expect("url should be valid");

        // Act
        let result = LocalClient::try_retrieve_code(&url);

        // Assert
        assert_eq!(result, code)
    }
}
