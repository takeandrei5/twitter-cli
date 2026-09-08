use std::cmp::max;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
};

use crate::{
    state::{ElementState, Mode},
    ui::BaseElement,
    utils::{ApplicationError, BORDER, SURFACE, TEXT_DIM, TEXT_MUTE},
};

const SHORTCUTS_READ_MODE: [(&str, &str); 7] = [
    ("j/k", "scroll"),
    ("r", "retweet"),
    ("l", "like/unlike"),
    ("Ctrl+w", "new post"),
    ("Ctrl+r", "refresh data"),
    ("o", "open"),
    ("Ctrl+x", "exit"),
];

const SHORTCUTS_WRITE_MODE: [(&str, &str); 5] = [
    ("Tab", "focus option"),
    ("Space", "toggle option"),
    ("Ctrl+s", "send"),
    ("Ctrl+w", "cancel"),
    ("Ctrl+x", "exit"),
];

const ROW_LIMIT: usize = 4;

pub struct BottomBarElement {
    lines: Vec<Line<'static>>,
}

impl BottomBarElement {
    pub fn new(mode: Mode) -> Self {
        let lines = match mode {
            Mode::Read => Self::create_lines(&SHORTCUTS_READ_MODE),
            Mode::Write => Self::create_lines(&SHORTCUTS_WRITE_MODE),
        };

        Self { lines }
    }

    fn create_spans_from_chunk(chunk: &[(&str, &str)]) -> Vec<Span<'static>> {
        let mut spans: Vec<Span<'static>> = Vec::with_capacity(1 + chunk.len() * 2);
        spans.push(Span::raw(" "));

        for &(key, action) in chunk {
            spans.push(Span::styled(
                format!(" {key} "),
                Style::default().fg(TEXT_DIM).bg(BORDER).bold(),
            ));

            spans.push(Span::styled(
                format!(" {action} "),
                Style::default().fg(TEXT_MUTE),
            ));
        }

        spans
    }

    fn create_lines(shortcuts: &[(&str, &str)]) -> Vec<Line<'static>> {
        shortcuts
            .chunks(ROW_LIMIT)
            .map(|chunk| {
                let spans: Vec<Span<'static>> = Self::create_spans_from_chunk(chunk);

                Line::from(spans).left_aligned().bg(SURFACE)
            })
            .collect()
    }
}

impl<T: ElementState> BaseElement<T> for BottomBarElement {
    fn draw(&mut self, frame: &mut Frame, area: Rect, _state: &T) -> Result<(), ApplicationError> {
        let lines_count =
            max(SHORTCUTS_READ_MODE.len(), SHORTCUTS_WRITE_MODE.len()).div_ceil(ROW_LIMIT);
        let layout = Layout::vertical(vec![Constraint::Fill(1); lines_count]);
        let line_areas: [Rect; 2] = area.layout(&layout);

        for (line, line_area) in self.lines.iter().zip(line_areas.iter()) {
            frame.render_widget(line, *line_area);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;
    use ratatui::style::{Style, Stylize};
    use ratatui::text::{Line, Span};
    use rstest::rstest;

    use crate::state::{Mode, ReadState};
    use crate::ui::bottom_bar_element::SHORTCUTS_READ_MODE;
    use crate::ui::{BaseElement, BottomBarElement};
    use crate::utils::{BORDER, SURFACE, TEXT_DIM, TEXT_MUTE};
    use crate::with_snapshot_settings;

    #[test]
    fn create_spans_from_chunk_should_create_correct_spans() {
        // Arrange
        let chunks: Vec<(&str, &str)> = SHORTCUTS_READ_MODE.into_iter().take(4).collect();
        let expected_result: Vec<Span<'_>> = vec![
            Span::raw(" "),
            Span::styled(" j/k ", Style::default().fg(TEXT_DIM).bg(BORDER).bold()),
            Span::styled(" scroll ", Style::default().fg(TEXT_MUTE)),
            Span::styled(" r ", Style::default().fg(TEXT_DIM).bg(BORDER).bold()),
            Span::styled(" retweet ", Style::default().fg(TEXT_MUTE)),
            Span::styled(" l ", Style::default().fg(TEXT_DIM).bg(BORDER).bold()),
            Span::styled(" like/unlike ", Style::default().fg(TEXT_MUTE)),
            Span::styled(" Ctrl+w ", Style::default().fg(TEXT_DIM).bg(BORDER).bold()),
            Span::styled(" new post ", Style::default().fg(TEXT_MUTE)),
        ];

        // Act
        let result = BottomBarElement::create_spans_from_chunk(chunks.as_slice());

        // Assert
        assert_eq!(result, expected_result);
    }

    #[test]
    fn create_lines_should_correct_lines() {
        // Arrange
        let first_spans: Vec<Span<'_>> = vec![
            Span::raw(" "),
            Span::styled(" j/k ", Style::default().fg(TEXT_DIM).bg(BORDER).bold()),
            Span::styled(" scroll ", Style::default().fg(TEXT_MUTE)),
            Span::styled(" r ", Style::default().fg(TEXT_DIM).bg(BORDER).bold()),
            Span::styled(" retweet ", Style::default().fg(TEXT_MUTE)),
            Span::styled(" l ", Style::default().fg(TEXT_DIM).bg(BORDER).bold()),
            Span::styled(" like/unlike ", Style::default().fg(TEXT_MUTE)),
            Span::styled(" Ctrl+w ", Style::default().fg(TEXT_DIM).bg(BORDER).bold()),
            Span::styled(" new post ", Style::default().fg(TEXT_MUTE)),
        ];

        let second_spans: Vec<Span<'_>> = vec![
            Span::raw(" "),
            Span::styled(" Ctrl+r ", Style::default().fg(TEXT_DIM).bg(BORDER).bold()),
            Span::styled(" refresh data ", Style::default().fg(TEXT_MUTE)),
            Span::styled(" o ", Style::default().fg(TEXT_DIM).bg(BORDER).bold()),
            Span::styled(" open ", Style::default().fg(TEXT_MUTE)),
            Span::styled(" Ctrl+x ", Style::default().fg(TEXT_DIM).bg(BORDER).bold()),
            Span::styled(" exit ", Style::default().fg(TEXT_MUTE)),
        ];
        let expected_result = [
            Line::from(first_spans).left_aligned().bg(SURFACE),
            Line::from(second_spans).left_aligned().bg(SURFACE),
        ];

        // Act
        let result = BottomBarElement::create_lines(&SHORTCUTS_READ_MODE);

        // Assert
        assert_eq!(result, expected_result);
    }

    #[rstest]
    #[case(Mode::Write)]
    #[case(Mode::Read)]
    fn draw_should_render_consistently(#[case] mode: Mode) {
        // Arrange
        let mut sut = BottomBarElement::new(mode);
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
                format!("bottom_bar_element_draw_should_render_consistently for mode {mode}"),
                terminal.backend()
            );
        });
    }
}
