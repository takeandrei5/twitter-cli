use crate::utils::ApplicationError;

#[derive(Debug)]
pub struct TwitterConfig {
    pub client_id: String,
    pub client_secret: String,
    pub authorize_url: String,
    pub redirect_url: String,
    pub token_url: String,
}

impl TwitterConfig {
    pub fn from_env() -> Result<Self, ApplicationError> {
        let client_id = dotenvy::var("TWITTER_CONSUMER_CLIENT_ID")
            .expect("Expected environment variable 'TWITTER_CONSUMER_CLIENT_ID' to be set");
        let client_secret = dotenvy::var("TWITTER_CONSUMER_SECRET")
            .expect("Expected environment variable 'TWITTER_CONSUMER_SECRET' to be set");
        let authorize_url = dotenvy::var("TWITTER_AUTHORIZE_URL")
            .expect("Expected environment variable 'TWITTER_AUTHORIZE_URL' to be set");
        let redirect_url = dotenvy::var("TWITTER_REDIRECT_URL")
            .expect("Expected environment variable 'TWITTER_REDIRECT_URL' to be set");
        let token_url = dotenvy::var("TWITTER_TOKEN_URL")
            .expect("Expected environment variable 'TWITTER_TOKEN_URL' to be set");

        Ok(Self {
            client_id,
            client_secret,
            authorize_url,
            redirect_url,
            token_url,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    #[should_panic]
    pub fn from_env_should_fail_when_twitter_consumer_client_id_is_missing() {
        // Arrange
        unsafe {
            env::remove_var("TWITTER_CONSUMER_CLIENT_ID");
            env::remove_var("TWITTER_CONSUMER_SECRET");
            env::remove_var("TWITTER_AUTHORIZE_URL");
            env::remove_var("TWITTER_REDIRECT_URL");
            env::remove_var("TWITTER_TOKEN_URL");
        }

        // Act
        let client_result = TwitterConfig::from_env();

        // Arrange
        assert_eq!(
            client_result.unwrap_err().to_string(),
            String::from("Expected environment variable 'TWITTER_CONSUMER_CLIENT_ID' to be set")
        );
    }

    #[test]
    #[should_panic]
    pub fn from_env_should_fail_when_twitter_consumer_secret_is_missing() {
        // Arrange
        unsafe {
            env::set_var("TWITTER_CONSUMER_CLIENT_ID", "test_1");
            env::remove_var("TWITTER_CONSUMER_SECRET");
            env::remove_var("TWITTER_AUTHORIZE_URL");
            env::remove_var("TWITTER_REDIRECT_URL");
            env::remove_var("TWITTER_TOKEN_URL");
        }

        // Act
        let client_result = TwitterConfig::from_env();

        // Arrange
        assert_eq!(
            client_result.unwrap_err().to_string(),
            String::from("Expected environment variable 'TWITTER_CONSUMER_SECRET' to be set")
        );

        clean_up();
    }

    #[test]
    #[should_panic]
    pub fn from_env_should_fail_when_twitter_authorize_url_is_missing() {
        // Arrange
        unsafe {
            env::set_var("TWITTER_CONSUMER_CLIENT_ID", "test");
            env::set_var("TWITTER_CONSUMER_SECRET", "test");
            env::remove_var("TWITTER_AUTHORIZE_URL");
            env::remove_var("TWITTER_REDIRECT_URL");
            env::remove_var("TWITTER_TOKEN_URL");
        }

        // Act
        let client_result = TwitterConfig::from_env();

        // Arrange
        assert_eq!(
            client_result.unwrap_err().to_string(),
            String::from("Expected environment variable 'TWITTER_AUTHORIZE_URL' to be set")
        );

        clean_up();
    }

    #[test]
    #[should_panic]
    pub fn from_env_should_fail_when_twitter_redirect_url_is_missing() {
        // Arrange
        unsafe {
            env::set_var("TWITTER_CONSUMER_CLIENT_ID", "test");
            env::set_var("TWITTER_CONSUMER_SECRET", "test");
            env::set_var("TWITTER_AUTHORIZE_URL", "test");
            env::remove_var("TWITTER_REDIRECT_URL");
            env::remove_var("TWITTER_TOKEN_URL");
        }

        // Act
        let client_result = TwitterConfig::from_env();

        // Arrange
        assert_eq!(
            client_result.unwrap_err().to_string(),
            String::from("Expected environment variable 'TWITTER_REDIRECT_URL' to be set")
        );

        clean_up();
    }

    #[test]
    #[should_panic]
    pub fn from_env_should_fail_when_twitter_token_url_is_missing() {
        // Arrange
        unsafe {
            env::set_var("TWITTER_CONSUMER_CLIENT_ID", "test");
            env::set_var("TWITTER_CONSUMER_SECRET", "test");
            env::set_var("TWITTER_AUTHORIZE_URL", "test");
            env::set_var("TWITTER_REDIRECT_URL", "test");
            env::remove_var("TWITTER_TOKEN_URL");
        }

        // Act
        let client_result = TwitterConfig::from_env();

        // Arrange
        assert_eq!(
            client_result.unwrap_err().to_string(),
            String::from("Expected environment variable 'TWITTER_TOKEN_URL' to be set")
        );

        clean_up();
    }

    #[test]
    pub fn from_env_should_not_fail() {
        // Arrange
        unsafe {
            env::set_var("TWITTER_CONSUMER_CLIENT_ID", "test");
            env::set_var("TWITTER_CONSUMER_SECRET", "test");
            env::set_var("TWITTER_AUTHORIZE_URL", "test");
            env::set_var("TWITTER_REDIRECT_URL", "test");
            env::set_var("TWITTER_TOKEN_URL", "test");
        }

        // Act
        let client_result = TwitterConfig::from_env();

        // Arrange
        assert!(client_result.is_ok());

        clean_up();
    }

    fn clean_up() {
        unsafe {
            env::remove_var("TWITTER_CONSUMER_CLIENT_ID");
            env::remove_var("TWITTER_CONSUMER_SECRET");
            env::remove_var("TWITTER_AUTHORIZE_URL");
            env::remove_var("TWITTER_REDIRECT_URL");
            env::remove_var("TWITTER_TOKEN_URL");
        }
    }
}
