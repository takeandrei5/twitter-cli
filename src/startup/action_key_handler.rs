use std::time::Duration;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    state::{Action, ActionOutcome, AppEvent, Mode},
    utils::ApplicationError,
};

pub(super) async fn handle_key_event(
    key: &KeyEvent,
    default_action: impl AsyncFnOnce() -> Result<Option<Action>, ApplicationError>,
) -> Result<Option<Action>, ApplicationError> {
    let action = match (key.modifiers, key.code) {
        (KeyModifiers::CONTROL, KeyCode::Char('w')) => Some(Action::SwitchMode),
        (KeyModifiers::CONTROL, KeyCode::Char('r')) => Some(Action::RefreshTweets),
        (KeyModifiers::CONTROL, KeyCode::Char('x')) => Some(Action::Quit),
        _ => default_action().await?,
    };

    Ok(action)
}

fn reset_view_state(event_sender: tokio::sync::mpsc::UnboundedSender<AppEvent>, event: AppEvent) {
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(2)).await;
        let _ = event_sender.send(event);
    });
}

pub fn handle_action_outcome(
    action_outcome: ActionOutcome,
    event_sender: tokio::sync::mpsc::UnboundedSender<AppEvent>,
    mode: &Mode,
    reset_read_view: impl FnOnce(),
    reset_write_view: impl FnOnce(),
    quit_app: impl FnOnce(),
) {
    match action_outcome {
        ActionOutcome::Quit => quit_app(),
        ActionOutcome::ResetReadView => {
            reset_read_view();
        }
        ActionOutcome::ResetWriteView => {
            reset_write_view();
        }
        ActionOutcome::DelayedResetView => match mode {
            Mode::Read => {
                reset_view_state(event_sender, AppEvent::ResetReadState);
            }
            Mode::Write => {
                reset_view_state(event_sender, AppEvent::ResetWriteState);
            }
        },
        ActionOutcome::None => {}
    };
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crossterm::event::KeyEventKind;

    #[tokio::test]
    #[rstest]
    #[case(KeyCode::Char('w'), KeyModifiers::CONTROL, Action::SwitchMode)]
    #[case(KeyCode::Char('r'), KeyModifiers::CONTROL, Action::RefreshTweets)]
    #[case(KeyCode::Char('x'), KeyModifiers::CONTROL, Action::Quit)]
    async fn handle_key_event_should_return_correct_result_when_known_controls_are_passed(
        #[case] code: KeyCode,
        #[case] modifiers: KeyModifiers,
        #[case] expected_result: Action,
    ) {
        // Arrange

        let key_event = KeyEvent::new_with_kind(code, modifiers, KeyEventKind::Press);

        let default_action = async || Ok(None);

        // Act
        let result = handle_key_event(&key_event, default_action)
            .await
            .expect("result should not be a failure")
            .expect("opton should not be none");

        // Assert
        assert_eq!(result, expected_result)
    }

    #[tokio::test]
    async fn handle_key_event_should_call_default_action_when_no_known_controls_are_passed() {
        // Arrange

        let key_event =
            KeyEvent::new_with_kind(KeyCode::Down, KeyModifiers::CONTROL, KeyEventKind::Press);

        let mut call_count = 0;
        let default_action = async || {
            call_count += 1;
            Ok(None)
        };

        // Act
        let _ = handle_key_event(&key_event, default_action).await;

        // Assert
        assert_eq!(call_count, 1);
    }

    
}
