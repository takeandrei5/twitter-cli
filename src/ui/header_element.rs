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
