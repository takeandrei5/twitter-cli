use std::io::Error;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};
use tracing::Level;

use crate::{
    api::{AuthClient, LocalClient, TwitterClient},
    app_state::{AppState, Mode, ViewAction},
    modes::{ReadView, View, WriteView},
    utils::ApplicationError,
};

mod api;
mod app_state;
mod custom_widgets;
mod modes;
mod ui;
mod utils;

#[tokio::main]
async fn main() -> Result<(), color_eyre::eyre::Error> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    color_eyre::install()?;

    let auth_client = AuthClient::new()?;
    let (pkce_code_verifier, csrf_token) = auth_client.start_login();

    let (code, state) = start_local_client().await?;

    let access_token = auth_client
        .handle_callback(pkce_code_verifier, code, state, &csrf_token)
        .await?;

    let twitter_client = TwitterClient::new(access_token);
    let app_state = AppState::new(twitter_client).await?;

    let mut terminal = ratatui::init();
    let result = app(&mut terminal, app_state).await;
    ratatui::restore();
    result?;
    Ok(())
}

async fn app(
    terminal: &mut DefaultTerminal,
    mut app_state: AppState,
) -> Result<(), ApplicationError> {
    let mut read_view = ReadView::new(&app_state);
    let mut write_view = WriteView::new(&app_state);

    loop {
        terminal.try_draw(|frame| {
            render(frame, &app_state, &mut read_view, &mut write_view)
                .map_err(|error| Error::other(error.to_string()))
        })?;

        let read_event = event::read()?;
        if let Event::Key(key) = read_event {
            match (key.modifiers, key.code) {
                (KeyModifiers::CONTROL, KeyCode::Char('w')) => {
                    switch_mode(&mut app_state, &mut read_view, &mut write_view).await
                }
                (KeyModifiers::CONTROL, KeyCode::Char('r')) => {
                    refresh_tweets(&mut app_state, &mut read_view).await?
                }
                (KeyModifiers::CONTROL, KeyCode::Char('x')) => return Ok(()),
                _ => {
                    if let Some(action) = handle_view_key(
                        key,
                        &read_event,
                        &mut app_state,
                        &mut read_view,
                        &mut write_view,
                    )
                    .await?
                    {
                        handle_view_action(action, &mut app_state, &mut read_view, &mut write_view)
                            .await;
                    }
                }
            }
        }
    }
}

async fn switch_mode(
    app_state: &mut AppState,
    read_view: &mut ReadView,
    write_view: &mut WriteView,
) {
    app_state.mode = match app_state.mode {
        Mode::Read => Mode::Write,
        Mode::Write => Mode::Read,
    };

    match &app_state.mode {
        Mode::Read => read_view.on_state_change(app_state),
        Mode::Write => write_view.on_state_change(app_state),
    }
}

async fn refresh_tweets(
    app_state: &mut AppState,
    read_view: &mut ReadView,
) -> Result<(), ApplicationError> {
    app_state.refresh_tweets().await?;
    read_view.reset();

    Ok(())
}

async fn handle_view_key(
    key: KeyEvent,
    event: &Event,
    app_state: &mut AppState,
    read_view: &mut ReadView,
    write_view: &mut WriteView,
) -> Result<Option<ViewAction>, ApplicationError> {
    match app_state.mode {
        Mode::Read => read_view.handle_key_event(key, event, app_state).await,
        Mode::Write => write_view.handle_key_event(key, event, app_state).await,
    }
}

async fn handle_view_action(
    action: ViewAction,
    app_state: &mut AppState,
    read_view: &mut ReadView,
    write_view: &mut WriteView,
) {
    match action {
        ViewAction::SwitchToRead => switch_mode(app_state, read_view, write_view).await,
    }
}

fn render(
    frame: &mut Frame,
    app_state: &AppState,
    read_view: &mut ReadView,
    write_view: &mut WriteView,
) -> Result<(), ApplicationError> {
    match app_state.mode {
        Mode::Read => read_view.render_view(frame, app_state),
        Mode::Write => write_view.render_view(frame, app_state),
    }
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
