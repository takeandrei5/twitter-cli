use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
};

use crate::{
    app_state::{AppState, Mode},
    ui::BaseElement,
    utils::{ApplicationError, BORDER, SURFACE, TEXT_DIM, TEXT_MUTE},
};

const SHORTCUTS_READ_MODE: [(&str, &str); 7] = [
    ("j/k", "scroll"),
    ("r", "repost"),
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

pub struct BottomBarElement {
    read_line: (Line<'static>, Line<'static>),
    write_line: (Line<'static>, Line<'static>),
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
    fn create_line(shortcuts: &[(&str, &str)]) -> (Line<'static>, Line<'static>) {
        let mut spans: Vec<(Span<'_>, Span<'_>)> = vec![];
        //  = vec![Span::raw(" ")]
        for &(key, action) in shortcuts {
            let new_item = (
                Span::styled(
                    format!(" {key} "),
                    Style::default().fg(TEXT_DIM).bg(BORDER).bold(),
                ),
                Span::styled(format!(" {action} "), Style::default().fg(TEXT_MUTE)),
            );

            spans.push(new_item);
        }

        let mut line1 = spans
            .drain(..4)
            .flat_map(|f| [f.0, f.1])
            .collect::<Vec<Span<'_>>>();
        let mut line2 = spans
            .into_iter()
            .flat_map(|f| [f.0, f.1])
            .collect::<Vec<Span<'_>>>();

        line1.insert(0, Span::raw(" "));
        line2.insert(0, Span::raw(" "));

        (
            Line::from(line1).left_aligned().bg(SURFACE),
            Line::from(line2).left_aligned().bg(SURFACE),
        )
    }
}

impl BaseElement for BottomBarElement {
    fn draw(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        app_state: &AppState,
    ) -> Result<(), ApplicationError> {
        let layout = Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]);
        let [line1_area, line2_area] = area.layout(&layout);

        let shortcuts_to_draw = match app_state.mode {
            Mode::Read => &self.read_line,
            Mode::Write => &self.write_line,
        };

        let (widget1, widget2) = shortcuts_to_draw;
        frame.render_widget(widget1, line1_area);
        frame.render_widget(widget2, line2_area);

        Ok(())
    }
}
