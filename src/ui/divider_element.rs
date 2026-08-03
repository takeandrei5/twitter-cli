use std::sync::LazyLock;

use ratatui::{
    Frame,
    layout::Rect,
    style::{Style, Stylize},
    text::Text,
    widgets::{Block, Widget},
};

use crate::{
    app_state::AppState,
    ui::BaseElement,
    utils::{ApplicationError, BLUE, TEXT_MUTE},
};

const LATEST_TEXT: &str = " ▸ Latest tweets";
static DIVIDER_WIDGET: LazyLock<Text<'_>> = LazyLock::new(|| {
    Text::from(LATEST_TEXT)
        .style(Style::new())
        .fg(TEXT_MUTE)
        .bg(BLUE)
});

pub struct DividerElement {
    _private: (),
}

impl DividerElement {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl BaseElement for DividerElement {
    fn draw(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        _app_state: &AppState,
    ) -> Result<(), ApplicationError> {
        frame.render_widget(&*DIVIDER_WIDGET, area);

        Ok(())
    }
}
