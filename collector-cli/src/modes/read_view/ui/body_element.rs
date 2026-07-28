use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::Block,
};

use crate::{
    app_state::AppState,
    custom_widgets::{Tweet, TweetState, TweetWidget},
    fake_data::create_fake_data,
    ui::BaseElement,
    utils::{ApplicationError, BG},
};

#[derive(Debug, Clone)]
pub struct BodyElement {
    tweets: Vec<Tweet>,
    current_list_index: usize,
}

impl BodyElement {
    pub fn new() -> Self {
        let tweets = create_fake_data();
        Self {
            tweets,
            current_list_index: 0,
        }
    }

    const fn reset(&mut self) {
        self.current_list_index = 0;
    }

    fn move_list_index_up(&mut self) {
        let cursor_moved_top = self.current_list_index.saturating_sub(1);
        self.current_list_index = self.clamp_list_index(cursor_moved_top);
    }

    fn move_list_index_down(&mut self) {
        let cursor_moved_down = self.current_list_index.saturating_add(1);
        self.current_list_index = self.clamp_list_index(cursor_moved_down);
    }

    fn clamp_list_index(&self, new_list_index: usize) -> usize {
        new_list_index.clamp(0, self.tweets.len())
    }
}

impl BaseElement for BodyElement {
    fn handle_key_event(&mut self, key: KeyEvent, _event: &Event, _app_state: &AppState) {
        match (key.modifiers, key.code) {
            (KeyModifiers::NONE, KeyCode::Char('j')) => {
                self.move_list_index_down();
            }
            (KeyModifiers::NONE, KeyCode::Char('k')) => {
                self.move_list_index_up();
            }
            _ => {}
        }
    }

    fn draw(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        _app_state: &AppState,
    ) -> Result<(), ApplicationError> {
        let widgets: Vec<TweetWidget> = self
            .tweets
            .clone()
            .into_iter()
            .map(|f| TweetWidget::new(f, TweetState::Normal))
            .collect();

        let container = Block::default().bg(BG);
        frame.render_widget(container.clone(), area);

        const TWEET_HEIGHT: u16 = 4;
        let visible_count: usize = (area.height / TWEET_HEIGHT).into();

        let scroll_offset = self
            .current_list_index
            .saturating_sub(visible_count.saturating_sub(1));

        let visible_widgets: Vec<TweetWidget> = (0..widgets.len())
            .zip(widgets)
            .skip(scroll_offset)
            .take(visible_count)
            .map(|(i, w)| {
                if i == self.current_list_index {
                    w.force_set_state(TweetState::Selected)
                } else {
                    w
                }
            })
            .collect();

        let layout = Layout::vertical(vec![
            Constraint::Length(TWEET_HEIGHT);
            visible_widgets.len()
        ])
        .split(area);

        for (item, layout_area) in visible_widgets.into_iter().zip(layout.iter()) {
            let inner = container.inner(*layout_area);
            frame.render_widget(container.clone(), *layout_area);
            frame.render_widget(item, inner);
        }

        Ok(())
    }
}
