use std::io::Error;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};
use tracing::Level;

use crate::{
    api::{AuthClient, LocalClient, TwitterClient},
    app_state::{AppState, Mode},
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

    let (code, state) = LocalClient::start_local_client()?;

    let access_token = auth_client
        .handle_callback(pkce_code_verifier, code, state, &csrf_token)
        .await?;

    let twitter_client = TwitterClient::new(access_token);

    let app_state = AppState::new(twitter_client).await?;

    // print!("\x1B[2J\x1B[1;1H"); // clean terminal

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
                    app_state.mode = match &app_state.mode {
                        Mode::Read => Mode::Write {
                            reply_to: read_view.prepare_for_state_change(&app_state),
                        },
                        Mode::Write { .. } => Mode::Read,
                    };

                    match &app_state.mode {
                        Mode::Read => read_view.on_state_change(&app_state),
                        Mode::Write { .. } => write_view.on_state_change(&app_state),
                    };
                }
                (KeyModifiers::CONTROL, KeyCode::Char('r')) => {
                    app_state.refresh_tweets().await?;
                    read_view.reset();
                    write_view.reset();
                }
                (KeyModifiers::NONE, KeyCode::Char('q')) => return Ok(()),
                _ => {
                    let mode = app_state.mode.clone();
                    match mode {
                        Mode::Read => {
                            read_view
                                .handle_key_event(key, &read_event, &mut app_state)
                                .await?;
                        }
                        Mode::Write { .. } => {
                            write_view
                                .handle_key_event(key, &read_event, &mut app_state)
                                .await?;
                        }
                    }
                }
            }
        }
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
        Mode::Write { .. } => write_view.render_view(frame, app_state),
    }
}
