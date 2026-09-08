use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Stylize},
    text::{Line, Span},
};

use crate::{
    state::{ReadState, StatusMessage},
    ui::BaseElement,
    utils::{ApplicationError, BLUE, GREEN, PINK, PURPLE, WHITE},
};

pub struct StatusBarElement;

impl StatusBarElement {
    fn get_status_message_styles(status_message: &StatusMessage) -> (&str, &str, Color) {
        let (label, message, color) = match status_message {
            StatusMessage::Liked => ("♥ LIKED", "Post liked", PINK),
            StatusMessage::Unliked => ("♡ UNLIKED", "Like removed", PINK),
            StatusMessage::Retweeted => ("↻ RETWEETED", "Post retweeted", PURPLE),
            StatusMessage::AlreadyRetweeted => (
                "^_^ ALREADY RETWEETED",
                "Post was retweeted already",
                PURPLE,
            ),
            StatusMessage::NewPostAdded => (" ✓ SUCCESS ", "New post added", GREEN),
        };

        (label, message, color)
    }

    fn render_default_status<'a>(tweet_count: usize) -> Line<'a> {
        let temp_fetched_message = format!(" fetched {} posts", tweet_count);

        Line::from(vec![
            Span::from(" READ ").bold(),
            Span::from(format!(" fetched {} posts ", temp_fetched_message)),
        ])
        .fg(WHITE)
        .bg(BLUE)
        .left_aligned()
    }

    fn render_custom_status<'a>((label, message, color): (&str, &str, Color)) -> Line<'a> {
        Line::from(vec![
            Span::from(format!(" {label} ")).bold(),
            Span::from(format!(" {message} ")),
        ])
        .fg(WHITE)
        .bg(color)
        .left_aligned()
    }
}

impl BaseElement<ReadState> for StatusBarElement {
    fn draw(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        read_state: &ReadState,
    ) -> Result<(), ApplicationError> {
        let widget = if let Some(status_message) = read_state.status_message() {
            let style = Self::get_status_message_styles(status_message);

            Self::render_custom_status(style)
        } else {
            Self::render_default_status(read_state.tweet_count())
        };

        frame.render_widget(widget, area);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use ratatui::{Terminal, backend::TestBackend};
    use rstest::rstest;

    use crate::with_snapshot_settings;

    use super::*;

    #[rstest]
    #[case(StatusMessage::Liked, "♥ LIKED", "Post liked", PINK)]
    #[case(StatusMessage::Unliked, "♡ UNLIKED", "Like removed", PINK)]
    #[case(StatusMessage::Retweeted, "↻ RETWEETED", "Post retweeted", PURPLE)]
    #[case(
        StatusMessage::AlreadyRetweeted,
        "^_^ ALREADY RETWEETED",
        "Post was retweeted already",
        PURPLE
    )]
    #[case(StatusMessage::NewPostAdded, " ✓ SUCCESS ", "New post added", GREEN)]
    fn get_status_message_styles_returns_expected_values(
        #[case] status: StatusMessage,
        #[case] expected_label: &str,
        #[case] expected_message: &str,
        #[case] expected_color: Color,
    ) {
        // Act
        let result = StatusBarElement::get_status_message_styles(&status);

        // Assert
        assert_eq!(result, (expected_label, expected_message, expected_color));
    }

    #[test]
    fn render_default_status_renders_cosistently() {
        // Assert
        let tweet_count = 500;

        // Act
        let result = StatusBarElement::render_default_status(tweet_count);

        // Assert
        with_snapshot_settings!({
            assert_debug_snapshot!(result);
        });
    }

    #[rstest]
    #[case("♥ LIKED", "Post liked", PINK)]
    #[case("♡ UNLIKED", "Like removed", PINK)]
    #[case("↻ RETWEETED", "Post retweeted", PURPLE)]
    #[case("^_^ ALREADY RETWEETED", "Post was retweeted already", PURPLE)]
    #[case(" ✓ SUCCESS ", "New post added", GREEN)]
    fn render_custom_status_renders_consistently(
        #[case] label: &str,
        #[case] message: &str,
        #[case] color: Color,
    ) {
        // Act
        let result = StatusBarElement::render_custom_status((label, message, color));

        // Assert
        with_snapshot_settings!({
            assert_debug_snapshot!(
                format!(
                    "render_custom_status_renders_consistently_label_{label}_message_{message}_color_{color}"
                ),
                result
            );
        });
    }

    #[rstest]
    #[case(Some(StatusMessage::Liked))]
    #[case(Some(StatusMessage::Unliked))]
    #[case(Some(StatusMessage::Retweeted))]
    #[case(Some(StatusMessage::AlreadyRetweeted))]
    #[case(Some(StatusMessage::NewPostAdded))]
    #[case(None)]
    fn read_status_bar_element_draw_renders_consistently(
        #[case] status_message: Option<StatusMessage>,
    ) {
        // Arrange
        let mut terminal =
            Terminal::new(TestBackend::new(40, 10)).expect("test terminal should be created");
        let mut status_bar_element_widget = StatusBarElement;
        let mut read_state = ReadState::new(vec![]);
        read_state.set_status_message(status_message.clone());

        // Act
        terminal
            .draw(|frame| {
                status_bar_element_widget
                    .draw(frame, frame.area(), &read_state)
                    .expect("status bar element should be rendered")
            })
            .expect("test terminal should be created");

        // Assert
        with_snapshot_settings!({
            assert_debug_snapshot!(
                format!(
                    "read_status_bar_element_draw_renders_consistently_status_message_{status_message:?}"
                ),
                terminal.backend()
            );
        });
    }
}
