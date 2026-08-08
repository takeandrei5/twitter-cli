use ratatui::{
    Frame,
    layout::Rect,
    style::{Style, Stylize},
    text::Text,
};

use crate::{
    app_state::AppState,
    ui::BaseElement,
    utils::{ApplicationError, BLUE, TEXT_MUTE},
};

pub struct DividerElement {
    text_widget: Text<'static>,
}

impl Default for DividerElement {
    fn default() -> Self {
        const LATEST_TEXT: &str = " ▸ Latest tweets";

        Self {
            text_widget: Text::from(LATEST_TEXT)
                .style(Style::new())
                .fg(TEXT_MUTE)
                .bg(BLUE),
        }
    }
}

impl BaseElement for DividerElement {
    fn draw(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        _app_state: &AppState,
    ) -> Result<(), ApplicationError> {
        frame.render_widget(&self.text_widget, area);

        Ok(())
    }
}
