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

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use ratatui::{Terminal, backend::TestBackend};
    use rstest::rstest;

    use crate::state::ReadState;
    use crate::with_snapshot_settings;
    use crate::{
        state::Mode,
        ui::{BaseElement, DividerElement},
    };

    #[rstest]
    #[case(Mode::Read)]
    #[case(Mode::Write)]
    fn draw_should_render_consistently(#[case] mode: Mode) {
        // Arrange
        let mut sut = DividerElement::new(mode);
        let read_state = ReadState::new(vec![]);

        let mut terminal =
            Terminal::new(TestBackend::new(20, 5)).expect("test terminal should be created");

        // Act
        terminal
            .draw(|frame| {
                sut.draw(frame, frame.area(), &read_state)
                    .expect("test terminal should be created")
            })
            .expect("test terminal should be created");

        // Assert
        with_snapshot_settings!({
            assert_snapshot!(
                format!("divider_element_draw_should_render_consistently_{:?}", mode),
                terminal.backend()
            );
        });
    }
}
