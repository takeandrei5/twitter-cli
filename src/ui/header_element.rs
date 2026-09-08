use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::Line,
    widgets::Block,
};

use crate::{
    state::ElementState,
    ui::BaseElement,
    utils::{ApplicationError, BLUE, SURFACE},
};

const TITLE: &str = "✦ TWITTER / TUI";

pub struct HeaderElement {
    left_widget: Line<'static>,
    background: Block<'static>,
    right_widget: Line<'static>,
}

impl HeaderElement {
    pub fn new(user_tag: &str) -> Self {
        Self {
            left_widget: Line::from(format!(" {} ", TITLE)).left_aligned(),
            background: Block::default().style(Style::default().bg(SURFACE).fg(BLUE).bold()),
            right_widget: Line::from(format!(" logged in as @{} ", user_tag)).right_aligned(),
        }
    }
}

impl<T: ElementState> BaseElement<T> for HeaderElement {
    fn draw(&mut self, frame: &mut Frame, area: Rect, _state: &T) -> Result<(), ApplicationError> {
        let layout = Layout::horizontal([Constraint::Fill(1); 2]);
        let [left_widget_area, right_widget_area] = area.layout(&layout);

        frame.render_widget(&self.background, area);
        frame.render_widget(&self.left_widget, left_widget_area);
        frame.render_widget(&self.right_widget, right_widget_area);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use ratatui::{Terminal, backend::TestBackend};

    use crate::state::ReadState;
    use crate::ui::BaseElement;
    use crate::ui::HeaderElement;
    use crate::with_snapshot_settings;

    #[test]
    fn draw_should_render_consistently() {
        // Arrange
        let mut sut = HeaderElement::new("Soup is good");
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
                format!("header_element_draw_should_render_consistently"),
                terminal.backend()
            );
        });
    }
}
