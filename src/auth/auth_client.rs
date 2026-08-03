use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, StandardTokenResponse, TokenResponse,
    TokenUrl,
    basic::{BasicClient, BasicTokenType},
    reqwest::{blocking::ClientBuilder, redirect::Policy},
};

use crate::utils::ApplicationError;

type TwitterOAuthClient = oauth2::Client<
    oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
    StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    oauth2::StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
    oauth2::StandardRevocableToken,
    oauth2::StandardErrorResponse<oauth2::RevocationErrorResponseType>,
    oauth2::EndpointSet,
    oauth2::EndpointNotSet,
    oauth2::EndpointNotSet,
    oauth2::EndpointNotSet,
    oauth2::EndpointSet,
>;

pub struct AuthClient {
    client: TwitterOAuthClient,
}

impl AuthClient {
    pub fn new() -> Result<Self, ApplicationError> {
        let client_id = dotenvy::var("TWITTER_CONSUMER_CLIENT_ID")?;
        let client_secret = dotenvy::var("TWITTER_CONSUMER_SECRET")?;
        let authorize_url = dotenvy::var("TWITTER_AUTHORIZE_URL")?;
        let redirect_url = dotenvy::var("TWITTER_REDIRECT_URL")?;
        let token_url = dotenvy::var("TWITTER_TOKEN_URL")?;

        let client = BasicClient::new(ClientId::new(client_id))
            .set_auth_uri(AuthUrl::new(authorize_url)?)
            .set_client_secret(ClientSecret::new(client_secret))
            .set_redirect_uri(RedirectUrl::new(redirect_url)?)
            .set_token_uri(TokenUrl::new(token_url)?);

        Ok(Self { client })
    }

    pub fn start_login(&self) -> (PkceCodeVerifier, CsrfToken) {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let mut temp_authorization_request = self
            .client
            .authorize_url(CsrfToken::new_random)
            .set_pkce_challenge(pkce_challenge);

        const REQUIRED_SCOPES: [&str; 6] = [
            "tweet.read",
            "tweet.write",
            "users.read",
            "like.write",
            "follows.read",
            "offline.access",
        ];

        for scope in REQUIRED_SCOPES {
            temp_authorization_request =
                temp_authorization_request.add_scope(Scope::new(scope.to_string()));
        }

        let (auth_url, csrf_token) = temp_authorization_request.url();

        println!("Returned the following values - auth_url {}", auth_url);

        (pkce_verifier, csrf_token)
    }

    pub fn handle_callback(
        &self,
        pkce_verifier: PkceCodeVerifier,
        code: String,
        state: String,
        expected_csrf_token: &CsrfToken,
    ) -> Result<String, ApplicationError> {
        if state != *expected_csrf_token.secret() {
            return Err(ApplicationError::CSRFTokenMismatch()); // adjust to your error type
        }

        let http_client = ClientBuilder::new()
            // Following redirects opens the client up to SSRF vulnerabilities.
            .redirect(Policy::none())
            .build()?;

        let token_result = self
            .client
            .exchange_code(AuthorizationCode::new(code))
            .set_pkce_verifier(pkce_verifier)
            .request(&http_client)?;

        Ok(token_result.access_token().secret().to_owned())
    }
}
