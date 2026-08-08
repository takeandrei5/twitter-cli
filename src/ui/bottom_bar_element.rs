use ratatui::{
    Frame,
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::{
    app_state::{AppState, Mode},
    ui::BaseElement,
    utils::{ApplicationError, BORDER, SURFACE, TEXT_DIM, TEXT_MUTE},
};

const SHORTCUTS_READ_MODE: [(&str, &str); 7] = [
    ("j/k", "scroll"),
    ("r", "retweet"),
    ("l", "like/unlike"),
    ("Ctrl+w", "quote post"),
    ("Ctrl+r", "refresh data"),
    ("o", "open"),
    ("q", "quit"),
];

const SHORTCUTS_WRITE_MODE: [(&str, &str); 5] = [
    ("CTRL+j/k", "reply scroll"),
    ("Ctrl+enter", "send"),
    ("Ctrl+w", "cancel"),
    ("Ctrl+r", "refresh data"),
    ("q", "quit"),
];

pub struct BottomBarElement {
    read_line: Line<'static>,
    write_line: Line<'static>,
}

impl Default for BottomBarElement {
    fn default() -> Self {
        Self {
            read_line: Self::create_line(&SHORTCUTS_READ_MODE),
            write_line: Self::create_line(&SHORTCUTS_WRITE_MODE),
        }
    }
}

impl BottomBarElement {
    fn create_line(shortcuts: &[(&str, &str)]) -> Line<'static> {
        let mut spans = vec![Span::raw(" ")];

        for &(key, action) in shortcuts {
            spans.push(Span::styled(
                format!(" {key} "),
                Style::default().fg(TEXT_DIM).bg(BORDER).bold(),
            ));

            spans.push(Span::styled(
                format!(" {action} "),
                Style::default().fg(TEXT_MUTE),
            ));
        }

        Line::from(spans)
    }
}

impl BaseElement for BottomBarElement {
    fn draw(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        app_state: &AppState,
    ) -> Result<(), ApplicationError> {
        let shortcuts_to_draw = match app_state.mode {
            Mode::Read => &self.read_line,
            Mode::Write { .. } => &self.write_line,
        };

        let widget = Paragraph::new(shortcuts_to_draw.to_owned())
            .left_aligned()
            .bg(SURFACE);
        frame.render_widget(widget, area);

        Ok(())
    }
}
