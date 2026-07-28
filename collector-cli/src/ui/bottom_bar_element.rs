use ratatui::{
    Frame,
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::{
    app_state::{AppState, Mode},
    ui::BaseElement,
    utils::{ApplicationError, BORDER, SURFACE, TEXT_DIM, TEXT_MUTE},
};

const SHORTCUTS_READ_MODE: [(&str, &str); 5] = [
    ("j/k", "scroll"),
    ("r", "retweet"),
    ("l", "like"),
    ("Ctrl+W", "write"),
    ("q", "quit"),
];

const SHORTCUTS_WRITE_MODE: [(&str, &str); 4] = [
    ("Ctrl+Enter", "send"),
    ("Ctrl+W", "cancel"),
    ("Ctrl+D", "discard"),
    ("CTRL+E/J", "reply scroll"),
];

pub struct BottomBarElement {
    _private: (),
}

impl BottomBarElement {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl BaseElement for BottomBarElement {
    fn draw(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        app_state: &AppState,
    ) -> Result<(), ApplicationError> {
        let shortcuts_to_draw = match app_state.mode {
            Mode::Read => &SHORTCUTS_READ_MODE[..],
            Mode::Write => &SHORTCUTS_WRITE_MODE[..],
        };

        let mut spans: Vec<Span<'static>> = vec![Span::raw(" ")];
        for (key, action) in shortcuts_to_draw {
            spans.push(Span::styled(
                format!(" {} ", key),
                Style::default().fg(TEXT_DIM).bg(BORDER).bold(),
            ));

            spans.push(Span::styled(
                format!(" {} ", action),
                Style::default().fg(TEXT_MUTE),
            ));
        }

        let widget = Paragraph::new(Line::from(spans)).left_aligned().bg(SURFACE);
        frame.render_widget(widget, area);

        Ok(())
    }
}
