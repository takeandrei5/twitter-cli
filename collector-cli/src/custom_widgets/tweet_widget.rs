use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget, Wrap},
};

use chrono::NaiveDateTime;

use crate::utils::{BG, GREEN, PINK, SELECT_BG, TEXT, TEXT_DIM, WHITE};

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

impl TweetWidget {
    pub fn new(tweet: Tweet, state: TweetState) -> Self {
        Self { tweet, state }
    }

    pub const fn force_set_state(mut self, new_state: TweetState) -> Self {
        self.state = new_state;

        self
    }
}

impl Widget for TweetWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let bg_color = if self.state == TweetState::Normal {
            BG
        } else {
            SELECT_BG
        };

        let vertical_chunks = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
        ]);

        let [
            first_vertical_zone_area,
            second_vertical_zone_area,
            third_vertical_zone_area,
        ] = area.layout(&vertical_chunks);

        let container = Block::default().bg(bg_color);

        container.render(area, buf);

        let first_zone = Line::from(vec![
            Span::from(self.tweet.name).style(Style::default().bold().fg(TEXT)),
            Span::from(format!(" {}", self.tweet.handle))
                .style(Style::default().not_bold().fg(TEXT_DIM)),
        ])
        .bg(bg_color);

        first_zone.render(first_vertical_zone_area, buf);

        let mut second_zone: Paragraph<'_> = Paragraph::new(format!(" {}", self.tweet.body))
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(WHITE))
            .bg(bg_color);

        let needed_lines_for_second_zone = second_zone.line_width() + 1; // 1 comes from the space formatting
        let cuttoff_point: usize =
            (second_vertical_zone_area.height * third_vertical_zone_area.width).into();
        if needed_lines_for_second_zone > cuttoff_point {
            let (trimmed_text, _) = self
                .tweet
                .body
                .split_at_checked(cuttoff_point - 13)
                .unwrap(); // 1 for the padding and 12 for the '...more info'

            second_zone = Paragraph::new(format!(" {}...more info", trimmed_text))
                .wrap(Wrap { trim: true })
                .style(Style::default().fg(WHITE))
                .bg(bg_color);
        }
        second_zone.render(second_vertical_zone_area, buf);

        let third_zone = Line::from(vec![
            Span::from(format!("↩{}", self.tweet.replies))
                .style(Style::default().bold().fg(TEXT_DIM)),
            Span::from(format!(" {} ", self.tweet.retweets))
                .style(Style::default().not_bold().fg(GREEN)),
            Span::from(format!("♥{}", self.tweet.likes))
                .style(Style::default().not_bold().fg(PINK)),
        ])
        .bg(bg_color);
        third_zone.render(third_vertical_zone_area, buf);
    }
}
