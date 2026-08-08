use ratatui::{
    Frame,
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, Span},
};

use crate::{
    app_state::AppState,
    ui::BaseElement,
    utils::{ApplicationError, BLUE, WHITE},
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
        app_state: &AppState,
    ) -> Result<(), ApplicationError> {
        let content = format!("fetched {} posts", app_state.tweets.len());

        let widget = Line::from_iter([
            Span::from("READ").bold(),
            Span::from(format!(" {} ", content)),
        ])
        .left_aligned()
        .style(Style::default().bg(BLUE).fg(WHITE));

        frame.render_widget(widget, area);

        Ok(())
    }
}
