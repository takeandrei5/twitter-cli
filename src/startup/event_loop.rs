use std::{
    io::Error,
    sync::atomic::{self, AtomicBool},
    time::Duration,
};

use crossterm::event::{self, Event, KeyEvent};
use ratatui::DefaultTerminal;

use crate::{
    modes::{ReadView, View, WriteView},
    state::{Action, AppState, Mode, ReadState, WriteState},
    utils::ApplicationError,
};

use super::action_key_handler::{handle_action_outcome, handle_key_event};

static BREAK_SIGNAL: AtomicBool = AtomicBool::new(false);

pub(super) async fn run_event_loop(
    terminal: &mut DefaultTerminal,
    mut app_state: AppState,
) -> Result<(), ApplicationError> {
    let username = app_state.username();
    let mut read_view = ReadView::new(username);
    let mut write_view = WriteView::new(username);

    loop {
        if BREAK_SIGNAL.load(atomic::Ordering::Relaxed) {
            return Ok(());
        }

        app_state.process_events();

        let mode = app_state.mode();
        let read_state = app_state.read_state();
        let write_state = app_state.write_state();

        terminal.try_draw(|frame| {
            match mode {
                Mode::Read => read_view.render_view(frame, read_state),
                Mode::Write => write_view.render_view(frame, write_state),
            }
            .map_err(|error| Error::other(error.to_string()))
        })?;

        if !event::poll(Duration::from_millis(100))? {
            continue;
        }

        let read_event = event::read()?;
        if let Event::Key(key) = read_event {
            let default_action = async || {
                handle_default_key_event(
                    key,
                    &read_event,
                    mode,
                    &mut read_view,
                    &mut write_view,
                    read_state,
                    write_state,
                )
                .await
            };

            let action = handle_key_event(&key, default_action).await?;

            if let Some(action) = action {
                let action_outcome = app_state.handle_action(action).await?;

                let event_sender = app_state.event_sender();
                handle_action_outcome(
                    action_outcome,
                    event_sender,
                    app_state.mode(),
                    || read_view.reset(),
                    || write_view.reset(),
                    || BREAK_SIGNAL.store(true, atomic::Ordering::Relaxed),
                );
            }
        }
    }

    async fn handle_default_key_event(
        key: KeyEvent,
        event: &Event,
        mode: &Mode,
        read_view: &mut ReadView,
        write_view: &mut WriteView,
        read_state: &ReadState,
        write_state: &WriteState,
    ) -> Result<Option<Action>, ApplicationError> {
        match mode {
            Mode::Read => read_view.handle_key_event(key, event, read_state).await,
            Mode::Write => write_view.handle_key_event(key, event, write_state).await,
        }
    }
}
