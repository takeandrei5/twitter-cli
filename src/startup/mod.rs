use crate::{
    api::{AuthClient, LocalClient, TwitterClient, TwitterConfig},
    state::AppState,
    utils::ApplicationError,
};

mod action_key_handler;
mod event_loop;

use event_loop::run_event_loop;

pub(super) async fn start_app() -> Result<(), ApplicationError> {
    let twitter_config = TwitterConfig::from_env()?;
    let auth_client = AuthClient::new(twitter_config)?;
    let (pkce_code_verifier, csrf_token) = auth_client.start_login();

    let (code, state) = start_local_client().await?;

    let access_token = auth_client
        .handle_callback(pkce_code_verifier, code, state, csrf_token)
        .await?;

    let twitter_client = TwitterClient::new(access_token, None);
    let app_state = AppState::new(twitter_client).await?;

    let mut terminal = ratatui::init();

    run_event_loop(&mut terminal, app_state).await
}

async fn start_local_client() -> Result<(String, String), ApplicationError> {
    tokio::task::spawn_blocking(LocalClient::start_local_client)
        .await
        .map_err(|error| {
            ApplicationError::UnexpectedError(std::io::Error::other(format!(
                "Callback listener task failed: {error}"
            )))
        })?
}
