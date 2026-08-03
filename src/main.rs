use std::io::Error;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};
use tracing::Level;

use crate::{
    app_state::{AppState, Mode},
    auth::AuthClient,
    local::LocalClient,
    modes::{ReadView, WriteView, view::View},
    utils::ApplicationError,
};

mod app_state;
mod auth;
mod custom_widgets;
mod fake_data;
mod local;
mod modes;
mod ui;
mod utils;

fn main() -> color_eyre::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    color_eyre::install()?;

    let app_client = AuthClient::new()?;
    let (pkce_code_verifier, csrf_token) = app_client.start_login();

    let (code, state) = LocalClient::start_local_client()?;
    let access_token = app_client.handle_callback(pkce_code_verifier, code, state, &csrf_token)?;

    // print!("\x1B[2J\x1B[1;1H");
    print!("{access_token}");
    // ratatui::run(|t| app(t, String::from("value")))?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal, access_token: String) -> Result<(), Error> {
    let tweet_count = fake_data::create_fake_data().len();
    let mut app_state = AppState::new(tweet_count, access_token);
    let mut read_view = ReadView::new();
    let mut write_view = WriteView::new();

    loop {
        terminal.try_draw(|frame| {
            match render(frame, &app_state, &mut read_view, &mut write_view) {
                Ok(_) => Ok(()),
                Err(_) => Err(Error::other(
                    "Unexpected error occurred inside the terminal UI.",
                )),
            }
        })?;

        let read_event = event::read()?;
        if let Event::Key(key) = read_event {
            match (key.modifiers, key.code) {
                (KeyModifiers::CONTROL, KeyCode::Char('w')) => {
                    app_state.mode = match app_state.mode {
                        Mode::Read => Mode::Write,
                        Mode::Write => Mode::Read,
                    };

                    match app_state.mode {
                        Mode::Read => read_view.on_state_change(&app_state),
                        Mode::Write => write_view.on_state_change(&app_state),
                    };
                }
                (KeyModifiers::NONE, KeyCode::Char('q')) => return Ok(()),
                _ => {
                    match app_state.mode {
                        Mode::Read => read_view.handle_key_event(key, &read_event, &app_state),
                        Mode::Write => write_view.handle_key_event(key, &read_event, &app_state),
                    };
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
        Mode::Write => write_view.render_view(frame, app_state),
    }
}
