use ratatui::{Frame, layout::Rect, style::Stylize, text::Text};

use crate::{
    state::{ElementState, Mode},
    ui::BaseElement,
    utils::{ApplicationError, BLUE, WHITE},
};

pub struct DividerElement {
    mode: Mode,
}

impl DividerElement {
    pub fn new(mode: Mode) -> Self {
        Self { mode }
    }
}

impl<T: ElementState> BaseElement<T> for DividerElement {
    fn draw(&mut self, frame: &mut Frame, area: Rect, _state: &T) -> Result<(), ApplicationError> {
        let widget_text = match self.mode {
            Mode::Read => " ▸ Latest tweets",
            Mode::Write => " ▸ Whatcha cooking there? 👀",
        };

        let widget = Text::from(widget_text).fg(WHITE).bg(BLUE);

        frame.render_widget(widget, area);

        Ok(())
    }
}
