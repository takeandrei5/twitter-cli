use async_trait::async_trait;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::Block,
};

use crate::{
    api::open_tweet,
    app_state::{AppState, ViewAction},
    custom_widgets::{TweetState, TweetWidget},
    ui::BaseElement,
    utils::{ApplicationError, BG},
};

#[derive(Debug, Clone, Default)]
pub struct BodyElement {
    current_list_index: usize,
}

impl BodyElement {
    fn move_list_index_up(&mut self, max_len: usize) {
        let cursor_moved_top = self.current_list_index.saturating_sub(1);
        self.current_list_index = self.clamp_list_index(cursor_moved_top, max_len);
    }

    fn move_list_index_down(&mut self, max_len: usize) {
        let cursor_moved_down = self.current_list_index.saturating_add(1);
        self.current_list_index = self.clamp_list_index(cursor_moved_down, max_len);
    }

    fn clamp_list_index(&self, new_list_index: usize, max_len: usize) -> usize {
        new_list_index.clamp(0, max_len.saturating_sub(1))
    }

    async fn toggle_like(&mut self, app_state: &mut AppState) -> Result<(), ApplicationError> {
        let current_tweet_data = app_state
            .tweets
            .get(self.current_list_index)
            .map(|tweet| (&tweet.id, tweet.liked));

        let (tweet_id, was_liked) = match current_tweet_data {
            Some((tweet_id, was_liked)) => (tweet_id, was_liked),
            _ => {
                tracing::debug!(
                    current_index = self.current_list_index,
                    tweets_count = app_state.tweets.len(),
                    "Could not handle LIKE/UNLIKE action from body element"
                );
                return Ok(());
            }
        };

        let user_id = &app_state.user_info.id;

        if was_liked {
            app_state
                .twitter_client
                .unlike_post(tweet_id, user_id)
                .await?;
        } else {
            app_state
                .twitter_client
                .like_post(tweet_id, user_id)
                .await?;
        }

        if let Some(tweet) = app_state.tweets.get_mut(self.current_list_index) {
            tweet.liked = !was_liked;
            tweet.likes = if was_liked {
                tweet.likes.saturating_sub(1)
            } else {
                tweet.likes.saturating_add(1)
            };
        }

        Ok(())
    }

    async fn repost_current(&mut self, app_state: &mut AppState) -> Result<(), ApplicationError> {
        let Some((tweet_id, was_reposted)) = app_state
            .tweets
            .get(self.current_list_index)
            .map(|tweet| (&tweet.id, tweet.retweeted))
        else {
            tracing::debug!(
                current_index = self.current_list_index,
                tweets_count = app_state.tweets.len(),
                "Could not handle REPOST action from body element"
            );
            return Ok(());
        };

        if was_reposted {
            return Ok(());
        }

        let user_id = &app_state.user_info.id;
        app_state
            .twitter_client
            .repost_post(tweet_id, user_id)
            .await?;

        if let Some(tweet) = app_state.tweets.get_mut(self.current_list_index) {
            tweet.retweeted = true;
        }

        Ok(())
    }

    fn open_url(&self, app_state: &AppState) {
        if let Some(current_tweet) = app_state.tweets.get(self.current_list_index) {
            if let Err(error) = open_tweet(current_tweet) {
                tracing::debug!(
                    tweet_id = %current_tweet.id,
                    error = %error,
                    "Could not open tweet in browser"
                );
            }
        } else {
            tracing::debug!(
                current_index = self.current_list_index,
                tweets_count = app_state.tweets.len(),
                "Could not open tweet: no tweet at current index"
            );
        }
    }
}

#[async_trait(?Send)]
impl BaseElement for BodyElement {
    async fn handle_key_event(
        &mut self,
        key: KeyEvent,
        _event: &Event,
        app_state: &mut AppState,
    ) -> Result<Option<ViewAction>, ApplicationError> {
        let tweets_len = app_state.tweets.len();
        match (key.modifiers, key.code) {
            (KeyModifiers::NONE, KeyCode::Char('j')) => self.move_list_index_down(tweets_len),
            (KeyModifiers::NONE, KeyCode::Char('k')) => self.move_list_index_up(tweets_len),
            (KeyModifiers::NONE, KeyCode::Char('l')) => self.toggle_like(app_state).await?,
            (KeyModifiers::NONE, KeyCode::Char('r')) => self.repost_current(app_state).await?,
            (KeyModifiers::NONE, KeyCode::Char('o')) => self.open_url(app_state),

            _ => {}
        }

        Ok(None)
    }

    fn draw(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        app_state: &AppState,
    ) -> Result<(), ApplicationError> {
        let widgets: Vec<TweetWidget> = app_state
            .tweets
            .iter()
            .map(|f| TweetWidget::new(f, TweetState::Normal))
            .collect();

        let container = Block::default().bg(BG);
        frame.render_widget(&container, area);

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

            frame.render_widget(&container, *layout_area);
            frame.render_widget(item, inner);
        }

        Ok(())
    }

    fn reset(&mut self) {
        self.current_list_index = 0;
    }
}
