use std::borrow::Cow;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget, Wrap},
};

use crate::{
    api::Tweet,
    utils::{BG, BLUE, GREEN, PINK, SELECT_BG, TEXT, TEXT_DIM, WHITE},
};

const MORE_TEXT: &str = "...more info";

#[derive(Debug, Clone)]
pub struct TweetWidget<'a> {
    tweet: &'a Tweet,
    state: TweetState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TweetState {
    Normal,
    Selected,
}

impl TweetState {
    fn get_bg_color(&self) -> Color {
        match self {
            Self::Normal => BG,
            Self::Selected => SELECT_BG,
        }
    }
}

impl<'a> TweetWidget<'a> {
    pub fn new(tweet: &'a Tweet, state: TweetState) -> Self {
        Self { tweet, state }
    }

    fn normalize_second_zone_body(&self, second_zone_area: &'a Rect) -> Cow<'_, str> {
        let max_allowed_len: usize = (second_zone_area.height * second_zone_area.width).into();

        let body: Cow<'_, str> = if self.tweet.body.len() > max_allowed_len {
            const SAFE_PADDING: usize = 2;
            let cutoff = max_allowed_len.saturating_sub(MORE_TEXT.len() + SAFE_PADDING);

            let trimmed = self
                .tweet
                .body
                .split_at_checked(cutoff)
                .map(|(s, _)| s)
                .unwrap_or(&self.tweet.body);

            Cow::Owned(format!("{trimmed}{MORE_TEXT}"))
        } else {
            Cow::Borrowed(self.tweet.body.as_ref())
        };

        body
    }

    fn render_first_zone(&self, area: Rect, buf: &mut Buffer) {
        let first_zone = Line::from(vec![
            Span::from(&self.tweet.name).style(Style::default().bold().fg(TEXT)),
            Span::from(format!(" {}", &self.tweet.handle))
                .style(Style::default().not_bold().fg(TEXT_DIM)),
        ]);

        first_zone.render(area, buf);
    }

    fn render_second_zone(&self, second_zone_area: Rect, buf: &mut Buffer) {
        let body = self.normalize_second_zone_body(&second_zone_area);

        let second_zone = Paragraph::new(Line::from(vec![Span::raw(" "), Span::raw(body)]))
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(WHITE));

        second_zone.render(second_zone_area, buf);
    }

    fn render_third_zone(&self, area: Rect, buf: &mut Buffer) {
        let liked_text = if self.tweet.liked {
            format!("♥{}", self.tweet.likes)
        } else {
            format!("♡{}", self.tweet.likes)
        };

        let (retweet_text, retweet_color) = if self.tweet.retweeted {
            ("●retweeted ", GREEN)
        } else {
            ("○retweeted ", TEXT_DIM)
        };

        let third_zone = Line::from(vec![
            Span::from(format!("↩{} ", self.tweet.replies)).fg(BLUE),
            Span::from(retweet_text).fg(retweet_color),
            Span::from(liked_text).fg(PINK),
        ]);
        third_zone.render(area, buf);
    }
}

impl Widget for TweetWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let [first_zone_area, second_zone_area, third_zone_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .areas(area);

        let bg_color = self.state.get_bg_color();
        let container = Block::default().bg(bg_color);

        container.render(area, buf);

        self.render_first_zone(first_zone_area, buf);
        self.render_second_zone(second_zone_area, buf);
        self.render_third_zone(third_zone_area, buf);
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;
    use insta::{assert_debug_snapshot, assert_snapshot};
    use ratatui::{Terminal, backend::TestBackend};
    use rstest::{fixture, rstest};

    use crate::tests_utils::with_snapshot_insta_settings;

    use super::*;

    #[fixture]
    fn simple_tweet() -> Tweet {
        Tweet {
            id: "test_id".to_owned(),
            name: "test_name".to_owned(),
            handle: "test_handle".to_owned(),
            time: NaiveDateTime::default(),
            body: "test_body".to_owned(),
            replies: 100,
            retweets: 100,
            likes: 100,
            liked: true,
            retweeted: false,
        }
    }

    #[fixture]
    fn long_tweet() -> Tweet {
        Tweet {
            id: "test_id".to_owned(),
            name: "test_name".to_owned(),
            handle: "test_handle".to_owned(),
            time: NaiveDateTime::default(),
            body: "THIS IS A SUPER VERY LONG TEXT AND CUTTOFF POINT IS INEVITABLE; THIS IS A SUPER VERY LONG TEXT AND CUTTOFF POINT IS INEVITABLE; THIS IS A SUPER VERY LONG TEXT AND CUTTOFF POINT IS INEVITABLE".to_owned(),
            replies: 100,
            retweets: 100,
            likes: 100,
            liked: true,
            retweeted: false,
        }
    }

    #[rstest]
    #[case(TweetState::Normal, BG)]
    #[case(TweetState::Selected, SELECT_BG)]
    fn get_bg_color_should_return_correct_color(
        #[case] current_tweet_state: TweetState,
        #[case] correct_color: Color,
    ) {
        // Act
        let color = TweetState::get_bg_color(&current_tweet_state);

        // Assert
        assert_eq!(color, correct_color);
    }

    #[rstest]
    fn normalize_second_zone_body_should_return_correct_result_when_cutoff_point_is_not_reached(
        simple_tweet: Tweet,
    ) {
        // Assert
        let rect = Rect::new(0, 0, 20, 1);
        let tweet_widget = TweetWidget::new(&simple_tweet, TweetState::Normal);

        // Act
        let result = tweet_widget.normalize_second_zone_body(&rect);

        // Assert
        assert_eq!(result, simple_tweet.body);
    }

    #[rstest]
    fn normalize_second_zone_body_should_return_correct_result_when_cutoff_point_is_reached(
        long_tweet: Tweet,
    ) {
        // Assert
        let rect = Rect::new(0, 0, 20, 1);
        let tweet_widget = TweetWidget::new(&long_tweet, TweetState::Normal);

        // Act
        let result = tweet_widget.normalize_second_zone_body(&rect);

        // Assert
        assert_eq!(result, "THIS I...more info");
    }

    #[rstest]
    fn render_first_zone_should_render_consistently(simple_tweet: Tweet) {
        // Arrange
        let tweet_widget = TweetWidget::new(&simple_tweet, TweetState::Normal);
        let area = Rect::new(0, 0, 40, 1);
        let mut buffer = Buffer::empty(area);

        // Act
        tweet_widget.render_first_zone(area, &mut buffer);

        // Assert
        with_snapshot_insta_settings(|| assert_debug_snapshot!(buffer));
    }

    #[rstest]
    fn render_second_zone_should_render_consistently_when_short_body_is_provided(
        simple_tweet: Tweet,
    ) {
        // Arrange
        let tweet_widget = TweetWidget::new(&simple_tweet, TweetState::Normal);
        let area = Rect::new(0, 0, 40, 1);
        let mut buffer = Buffer::empty(area);

        // Act
        tweet_widget.render_second_zone(area, &mut buffer);

        // Assert
        with_snapshot_insta_settings(|| assert_debug_snapshot!(buffer));
    }

    #[rstest]
    fn render_second_zone_should_render_consistently_when_long_body_is_provided(long_tweet: Tweet) {
        // Arrange
        let tweet_widget = TweetWidget::new(&long_tweet, TweetState::Normal);
        let area = Rect::new(0, 0, 40, 1);
        let mut buffer = Buffer::empty(area);

        // Act
        tweet_widget.render_second_zone(area, &mut buffer);

        // Assert
        with_snapshot_insta_settings(|| assert_debug_snapshot!(buffer));
    }

    #[rstest]
    #[case(false, false)]
    #[case(false, true)]
    #[case(true, false)]
    #[case(true, true)]
    fn render_widget_should_render_consistently_on_small_screens(
        mut long_tweet: Tweet,
        #[case] liked: bool,
        #[case] retweeted: bool,
    ) {
        // Arrange
        long_tweet.liked = liked;
        long_tweet.retweeted = retweeted;

        let tweet_widget = TweetWidget::new(&long_tweet, TweetState::Normal);
        let mut terminal = Terminal::new(TestBackend::new(20, 5)).unwrap();

        // Act
        terminal
            .draw(|frame| frame.render_widget(tweet_widget, frame.area()))
            .unwrap();

        // Assert
        with_snapshot_insta_settings(|| {
            assert_snapshot!(
                format!(
                    "render_widget_should_render_consistently_on_small_screens_liked_{liked}_retweeted_{retweeted}"
                ),
                terminal.backend()
            );
        });
    }

    #[rstest]
    #[case(false, false)]
    #[case(false, true)]
    #[case(true, false)]
    #[case(true, true)]
    fn render_widget_should_render_consistently_on_medium_screens(
        mut long_tweet: Tweet,
        #[case] liked: bool,
        #[case] retweeted: bool,
    ) {
        // Arrange
        long_tweet.liked = liked;
        long_tweet.retweeted = retweeted;

        let tweet_widget = TweetWidget::new(&long_tweet, TweetState::Normal);
        let mut terminal = Terminal::new(TestBackend::new(20, 10)).unwrap();

        // Act
        terminal
            .draw(|frame| frame.render_widget(tweet_widget, frame.area()))
            .unwrap();

        // Assert
        with_snapshot_insta_settings(|| {
            assert_snapshot!(
                format!(
                    "render_widget_should_render_consistently_on_medium_screens_liked_{liked}_retweeted_{retweeted}"
                ),
                terminal.backend()
            );
        });
    }

    #[rstest]
    #[case(false, false)]
    #[case(false, true)]
    #[case(true, false)]
    #[case(true, true)]
    fn render_widget_should_render_consistently_on_large_screens(
        mut long_tweet: Tweet,
        #[case] liked: bool,
        #[case] retweeted: bool,
    ) {
        // Arrange
        long_tweet.liked = liked;
        long_tweet.retweeted = retweeted;

        let tweet_widget = TweetWidget::new(&long_tweet, TweetState::Normal);
        let mut terminal = Terminal::new(TestBackend::new(40, 10)).unwrap();

        // Act
        terminal
            .draw(|frame| frame.render_widget(tweet_widget, frame.area()))
            .unwrap();

        // Assert
        with_snapshot_insta_settings(|| {
            assert_snapshot!(
                format!(
                    "render_widget_should_render_consistently_on_large_screens_liked_{liked}_retweeted_{retweeted}"
                ),
                terminal.backend()
            );
        });
    }
}
