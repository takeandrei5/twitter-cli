use ratatui::{Frame, layout::Rect, style::Stylize, text::Text};

use crate::{
    app_state::{AppState, Mode},
    ui::BaseElement,
    utils::{ApplicationError, BLUE, TEXT_MUTE},
};

pub struct DividerElement;

impl BaseElement for DividerElement {
    fn draw(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        app_state: &AppState,
    ) -> Result<(), ApplicationError> {
        let widget_text = match app_state.mode {
            Mode::Read => " ▸ Latest tweets",
            Mode::Write => " ▸ Whatcha cooking there? 👀",
        };

        let widget = Text::from(widget_text).fg(TEXT_MUTE).bg(BLUE);

        frame.render_widget(widget, area);

        Ok(())
    }
}
