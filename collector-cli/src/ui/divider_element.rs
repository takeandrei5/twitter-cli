use ratatui::{
    Frame,
    layout::Rect,
    style::{Style, Stylize},
    text::Text,
    widgets::Block,
};

use crate::{
    app_state::AppState,
    ui::BaseElement,
    utils::{ApplicationError, BLUE, TEXT_MUTE},
};

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
        frame.render_widget(Block::new().style(Style::default()).bg(BLUE), area);
        frame.render_widget(
            Text::from(" ▸ Latest tweets")
                .style(Style::new())
                .fg(TEXT_MUTE),
            area,
        );

        Ok(())
    }
}
