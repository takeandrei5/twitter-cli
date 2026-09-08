use async_trait::async_trait;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::Block,
};

use super::custom_widgets::{TweetState, TweetWidget};

use crate::{
    api::Tweet,
    state::{Action, ReadState},
    ui::BaseElement,
    utils::{ApplicationError, BG},
};

const TWEET_HEIGHT: u16 = 4;

#[derive(Debug, Clone, Default)]
pub struct BodyElement {
    current_list_index: usize,
}

impl BodyElement {
    fn move_list_index_up(&mut self, max_len: usize) {
        self.current_list_index =
            self.clamp_list_index(self.current_list_index.saturating_sub(1), max_len);
    }

    fn move_list_index_down(&mut self, max_len: usize) {
        self.current_list_index =
            self.clamp_list_index(self.current_list_index.saturating_add(1), max_len);
    }

    fn clamp_list_index(&self, new_list_index: usize, max_len: usize) -> usize {
        new_list_index.clamp(0, max_len.saturating_sub(1))
    }

    fn like_action(&self, state: &ReadState) -> Option<Action> {
        state
            .tweet(self.current_list_index)
            .map(|tweet| Action::LikeTweet {
                tweet_id: tweet.id.clone(),
                was_liked: tweet.liked,
            })
    }

    fn retweet_action(&self, state: &ReadState) -> Option<Action> {
        state
            .tweet(self.current_list_index)
            .map(|tweet| Action::RetweetTweet {
                tweet_id: tweet.id.clone(),
            })
    }

    fn open_action(&self, state: &ReadState) -> Option<Action> {
        let tweet = state.tweet(self.current_list_index);

        tweet.cloned().map(Action::OpenTweet)
    }

    fn calculate_visible_count_and_scroll_offset(&self, area: &Rect) -> (usize, usize) {
        let visible_count = (area.height / TWEET_HEIGHT) as usize;
        let scroll_offset = self
            .current_list_index
            .saturating_sub(visible_count.saturating_sub(1));

        (visible_count, scroll_offset)
    }

    fn create_tweet_widgets<'a>(
        &self,
        tweets: &'a [Tweet],
        scroll_offset: usize,
        visible_count: usize,
    ) -> Vec<TweetWidget<'a>> {
        let visible_widgets: Vec<TweetWidget> = tweets
            .iter()
            .skip(scroll_offset)
            .take(visible_count)
            .enumerate()
            .map(|(index, tweet)| {
                if index == self.current_list_index {
                    TweetWidget::new(tweet, TweetState::Selected)
                } else {
                    TweetWidget::new(tweet, TweetState::Normal)
                }
            })
            .collect();

        visible_widgets
    }

    fn render_tweet_widgets_layout<'a>(
        visible_widgets: Vec<TweetWidget<'a>>,
        container: Block<'a>,
        area: Rect,
        frame: &mut Frame,
    ) {
        let layout = Layout::vertical(vec![
            Constraint::Length(TWEET_HEIGHT);
            visible_widgets.len()
        ])
        .split(area);

        let zipped_tweet_area_widgets = visible_widgets.into_iter().zip(layout.iter().copied());

        for (item, layout_area) in zipped_tweet_area_widgets {
            let inner = container.inner(layout_area);
            frame.render_widget(&container, layout_area);
            frame.render_widget(item, inner);
        }
    }
}

#[async_trait(?Send)]
impl BaseElement<ReadState> for BodyElement {
    async fn handle_key_event(
        &mut self,
        key: KeyEvent,
        _event: &Event,
        state: &ReadState,
    ) -> Result<Option<Action>, ApplicationError> {
        let action = match (key.modifiers, key.code) {
            (KeyModifiers::NONE, KeyCode::Char('j')) => {
                self.move_list_index_down(state.tweet_count());
                None
            }
            (KeyModifiers::NONE, KeyCode::Char('k')) => {
                self.move_list_index_up(state.tweet_count());
                None
            }
            (KeyModifiers::NONE, KeyCode::Char('l')) => self.like_action(state),
            (KeyModifiers::NONE, KeyCode::Char('r')) => self.retweet_action(state),
            (KeyModifiers::NONE, KeyCode::Char('o')) => self.open_action(state),
            (KeyModifiers::CONTROL, KeyCode::Char('r')) => Some(Action::RefreshTweets),
            _ => None,
        };

        Ok(action)
    }

    fn draw(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        state: &ReadState,
    ) -> Result<(), ApplicationError> {
        let container = Block::default().bg(BG);
        frame.render_widget(&container, area);

        let (visible_count, scroll_offset) = self.calculate_visible_count_and_scroll_offset(&area);

        let tweets = state.tweets();
        let visible_widgets = self.create_tweet_widgets(tweets, scroll_offset, visible_count);

        Self::render_tweet_widgets_layout(visible_widgets, container, area, frame);

        Ok(())
    }

    fn reset(&mut self) {
        self.current_list_index = 0;
    }
}
