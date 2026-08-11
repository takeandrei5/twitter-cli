use ratatui::{
    Frame,
    layout::Rect,
    style::Stylize,
    text::{Line, Span},
};

use crate::{
    app_state::{AppState, Mode, StatusMessage},
    ui::BaseElement,
    utils::{ApplicationError, BLUE, GREEN, PINK, PURPLE, RED, WHITE},
};

pub struct StatusBarElement;

impl BaseElement for StatusBarElement {
    fn draw(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        app_state: &AppState,
    ) -> Result<(), ApplicationError> {
        let widget = if let Some(status_message) = &app_state.status_message {
            let (label, message, color) = match status_message {
                StatusMessage::Liked => ("♥ LIKED", "Post liked", PINK),
                StatusMessage::Unliked => ("♡ UNLIKED", "Like removed", PINK),
                StatusMessage::Reposted => ("↻ REPOSTED", "Post reposted", PURPLE),
                StatusMessage::NewPostAdded => ("✓ SUCCESS", "New post added!", GREEN),
            };

            Line::from(vec![
                Span::from(format!(" {label} ")).bold(),
                Span::from(format!(" {message} ")),
            ])
            .fg(WHITE)
            .bg(color)
            .left_aligned()
        } else {
            match app_state.mode {
                Mode::Write => Line::from(vec![
                    Span::from(" WRITE ").bold(),
                    Span::from(" insert text normally · Ctrl+W to cancel "),
                ])
                .left_aligned()
                .bg(RED)
                .fg(WHITE),

                Mode::Read => Line::from(vec![
                    Span::from(" READ ").bold(),
                    Span::from(format!(" fetched {} posts ", app_state.tweets.len())),
                ])
                .left_aligned()
                .bg(BLUE)
                .fg(WHITE),
            }
        };

        frame.render_widget(widget, area);
        Ok(())
    }
}
