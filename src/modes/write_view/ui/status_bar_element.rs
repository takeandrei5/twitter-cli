use std::sync::LazyLock;

use ratatui::{
    Frame,
    layout::Rect,
    style::Stylize,
    text::{Line, Span},
};

use crate::{
    state::{StatusMessage, WriteState},
    ui::BaseElement,
    utils::{ApplicationError, GREEN, RED, WHITE},
};

static DEFAULT_WIDGET: LazyLock<Line<'static>> = LazyLock::new(|| {
    Line::from(vec![
        Span::from(" WRITE ").bold(),
        Span::from(" insert text normally · Ctrl+W to cancel "),
    ])
    .left_aligned()
    .bg(RED)
    .fg(WHITE)
});

static SUCCESS_WIDGET: LazyLock<Line<'static>> = LazyLock::new(|| {
    Line::from(vec![
        Span::from(" ✓ SUCCESS ").bold(),
        Span::from("New post added! "),
    ])
    .left_aligned()
    .fg(WHITE)
    .bg(GREEN)
});

pub struct StatusBarElement;

impl BaseElement<WriteState> for StatusBarElement {
    fn draw(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        app_state: &WriteState,
    ) -> Result<(), ApplicationError> {
        let mut widget = &*DEFAULT_WIDGET;

        if let Some(status_message) = app_state.status_message()
            && *status_message == StatusMessage::NewPostAdded
        {
            widget = &SUCCESS_WIDGET
        }

        frame.render_widget(widget, area);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use ratatui::{Terminal, backend::TestBackend};
    use rstest::rstest;

    use super::*;
    use crate::with_snapshot_settings;

    #[rstest]
    #[case(Some(StatusMessage::Liked))]
    #[case(Some(StatusMessage::Unliked))]
    #[case(Some(StatusMessage::Retweeted))]
    #[case(Some(StatusMessage::AlreadyRetweeted))]
    #[case(Some(StatusMessage::NewPostAdded))]
    #[case(None)]
    fn write_status_bar_element_draw_renders_consistently(
        #[case] status_message: Option<StatusMessage>,
    ) {
        // Arrange
        let mut terminal =
            Terminal::new(TestBackend::new(40, 2)).expect("test terminal should be created");
        let mut status_bar_element_widget = StatusBarElement;
        let mut write_state = WriteState::default();
        write_state.set_status_message(status_message.clone());

        // Act
        terminal
            // .draw(|frame| frame.render_widget(status_bar_element_widget, frame.area()))
            .draw(|frame| {
                status_bar_element_widget
                    .draw(frame, frame.area(), &write_state)
                    .expect("status bar element should be rendered")
            })
            .expect("test terminal should be created");

        // Assert
        with_snapshot_settings!({
            assert_debug_snapshot!(
                format!("write_status_bar_element_draw_renders_consistently_{status_message:?}"),
                terminal.backend()
            );
        });
    }
}
