use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::Line,
    widgets::Block,
};

use crate::{
    app_state::AppState,
    ui::BaseElement,
    utils::{BLUE, SURFACE},
};

const TITLE: &str = "✦ TWITTER / TUI";
const RIGHT_AREA_TEXT: &str = "logged in as @{user_tag}";

pub struct HeaderElement {
    _private: (),
}

impl HeaderElement {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl BaseElement for HeaderElement {
    fn draw(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        app_state: &AppState,
    ) -> Result<(), crate::utils::ApplicationError> {
        let layout = Layout::horizontal([Constraint::Min(0); 2]).split(area);

        let background = Block::default().style(Style::default().bg(SURFACE).fg(BLUE).bold());
        frame.render_widget(background, area);

        let left_widget = Line::from(format!(" {} ", TITLE)).left_aligned();
        frame.render_widget(left_widget, layout[0]);

        let user_tag = app_state.user_tag.as_ref().unwrap();

        let right_area_content = RIGHT_AREA_TEXT.replace("{user_tag}", user_tag);
        let right_widget = Line::from(format!(" {} ", right_area_content)).right_aligned();
        frame.render_widget(right_widget, layout[1]);

        Ok(())
    }
}
