use std::borrow::Cow;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget, Wrap},
};

use chrono::NaiveDateTime;

use crate::utils::{BG, GREEN, PINK, SELECT_BG, TEXT, TEXT_DIM, WHITE};

const MORE_TEXT: &str = "...more info";

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Tweet {
    pub name: String,
    pub handle: String,
    pub time: NaiveDateTime,
    pub body: String,
    pub replies: u32,
    pub retweets: u32,
    pub likes: u32,
    pub liked: bool,
    pub retweeted: bool,
}

#[derive(Debug, Clone)]
pub struct TweetWidget {
    tweet: Tweet,
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

impl TweetWidget {
    pub fn new(tweet: Tweet, state: TweetState) -> Self {
        Self { tweet, state }
    }

    pub const fn force_set_state(mut self, new_state: TweetState) -> Self {
        self.state = new_state;

        self
    }

    fn render_first_zone(&self, area: Rect, buf: &mut Buffer) {
        let first_zone = Line::from_iter([
            Span::from(&self.tweet.name).style(Style::default().bold().fg(TEXT)),
            Span::from(format!(" {}", &self.tweet.handle))
                .style(Style::default().not_bold().fg(TEXT_DIM)),
        ]);

        first_zone.render(area, buf);
    }

    fn render_second_zone(&self, second_zone_area: Rect, buf: &mut Buffer) {
        let cuttoff_point: usize = (second_zone_area.height * second_zone_area.width).into();

        let body: Cow<'_, str> = if self.tweet.body.len() > cuttoff_point {
            let cutoff = cuttoff_point.saturating_sub(MORE_TEXT.len() + 1);

            let trimmed = self
                .tweet
                .body
                .split_at_checked(cutoff)
                .map(|(s, _)| s)
                .unwrap_or(&self.tweet.body);

            Cow::Owned(format!("{trimmed}{MORE_TEXT}"))
        } else {
            Cow::Borrowed(self.tweet.body.as_str())
        };

        let second_zone =
            Paragraph::new(Line::from_iter([Span::raw(" "), Span::raw(body.as_ref())]))
                .wrap(Wrap { trim: true })
                .style(Style::default().fg(WHITE));

        second_zone.render(second_zone_area, buf);
    }

    fn render_third_zone(&self, area: Rect, buf: &mut Buffer) {
        let third_zone = Line::from_iter([
            Span::from(format!("↩{}", self.tweet.replies))
                .style(Style::default().bold().fg(TEXT_DIM)),
            Span::from(format!(" {} ", self.tweet.retweets))
                .style(Style::default().not_bold().fg(GREEN)),
            Span::from(format!("♥{}", self.tweet.likes))
                .style(Style::default().not_bold().fg(PINK)),
        ]);
        third_zone.render(area, buf);
    }
}

impl Widget for TweetWidget {
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
