use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Span},
};

use crate::{
    app_state::AppState,
    ui::BaseElement,
    utils::{ApplicationError, BLUE, WHITE},
};

#[derive(Debug, Clone, Copy)]
struct StatusText {
    label: &'static str,
    description: &'static str,
    background_color: Color,
}

const STATUS_TEXT: StatusText = StatusText {
    label: "READ",
    description: "fetched {tweet_count} posts",
    background_color: BLUE,
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
        let content = STATUS_TEXT
            .description
            .replace("{tweet_count}", &app_state.tweet_count.to_string());

        let widget = Line::from(vec![
            Span::from(format!(" {} ", STATUS_TEXT.label)).bold(),
            Span::from(format!(" {} ", content)),
        ])
        .left_aligned()
        .style(Style::default().bg(STATUS_TEXT.background_color).fg(WHITE));

        frame.render_widget(widget, area);

        Ok(())
    }
}
