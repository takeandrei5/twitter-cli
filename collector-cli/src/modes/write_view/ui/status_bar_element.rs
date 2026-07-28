use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Stylize},
    text::{Line, Span},
};

use crate::{
    app_state::AppState,
    ui::BaseElement,
    utils::{ApplicationError, RED, WHITE},
};

#[derive(Debug, Clone, Copy)]
struct StatusText {
    label: &'static str,
    description: &'static str,
    background_color: Color,
}

const STATUS_TEXT: StatusText = StatusText {
    label: "WRITE",
    description: "insert text normally · Ctrl+W to cancel",
    background_color: RED,
};

pub struct StatusBarElement {
    _private: (),
}

impl StatusBarElement {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl BaseElement for StatusBarElement {
    fn draw(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        _app_state: &AppState,
    ) -> Result<(), ApplicationError> {
        let widget = Line::from(vec![
            Span::from(format!(" {} ", STATUS_TEXT.label)).bold(),
            Span::from(format!(" {} ", STATUS_TEXT.description)),
        ])
        .left_aligned()
        .bg(STATUS_TEXT.background_color)
        .fg(WHITE);

        frame.render_widget(widget, area);
        Ok(())
    }
}
