use crate::{
    app_state::{AppState, Mode},
    ui::BaseElement,
    utils::{ApplicationError, BG, PINK, TEXT, TEXT_DIM},
};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Position, Rect},
    style::Stylize,
    widgets::{Block, BorderType, Paragraph, Wrap},
};
use tui_input::{Input, backend::crossterm::EventHandler};

#[derive(Debug, Clone)]
pub struct BodyElement {
    input: Input,
    scroll_tweet_body: usize,
    upper_scroll_limit_boundary: usize,
}

impl BodyElement {
    pub fn new() -> Self {
        Self {
            input: Input::new(String::new()),
            scroll_tweet_body: 0,
            upper_scroll_limit_boundary: 0,
        }
    }

    fn move_scroll_up(&mut self) {
        if self.scroll_tweet_body > 0 {
            self.scroll_tweet_body -= 1;
        }
    }

    fn move_scroll_down(&mut self) {
        if self.scroll_tweet_body < self.upper_scroll_limit_boundary {
            self.scroll_tweet_body += 1;
        }
    }

    fn submit_message(&mut self) {
        self.input.reset();
    }
}

impl BaseElement for BodyElement {
    fn handle_state_change(&mut self, app_state: &AppState) {
        if app_state.mode == Mode::Write {
            self.input.reset();
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent, event: &Event, _app_state: &AppState) {
        match (key.modifiers, key.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('j')) => {
                self.move_scroll_down();
            }
            (KeyModifiers::CONTROL, KeyCode::Char('k')) => {
                self.move_scroll_up();
            }
            (KeyModifiers::CONTROL, KeyCode::Enter) => {
                self.submit_message();
            }
            _ => {
                self.input.handle_event(event);
            }
        }
    }

    fn draw(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        app_state: &AppState,
    ) -> Result<(), ApplicationError> {
        let tweet = app_state.reply_to_user_tweet.as_ref().unwrap();

        let container = Block::default().bg(BG);
        frame.render_widget(container, area);

        let layout = Layout::vertical([Constraint::Min(0), Constraint::Max(3)]);
        let [reply_area, input_area] = area.layout(&layout);

        let top_block = Block::bordered()
            .fg(TEXT_DIM)
            .border_type(BorderType::Rounded)
            .title(format!(
                "↩ replying to {} · posted at {}",
                tweet.handle,
                tweet.time.format("%Y-%m-%d %H:%M")
            ));

        let tweet = app_state.reply_to_user_tweet.as_ref().unwrap();
        let inner_top_block = top_block.inner(reply_area);

        /*
        3 here means - 1 for border T, 1 element, 1 border B
        MAX(3) because if area is < 3 we dont want to underflow
        we always extract 3 because that's the reserved space
        */
        let tweet_body_height = inner_top_block.height.max(3);

        let inner_top_block_tweet_body = Paragraph::new(&*tweet.body)
            .wrap(Wrap { trim: true })
            .scroll((self.scroll_tweet_body as u16, 0))
            .fg(TEXT);

        self.upper_scroll_limit_boundary = inner_top_block_tweet_body
            .line_count(inner_top_block.width)
            .saturating_sub(tweet_body_height as usize);

        frame.render_widget(top_block, reply_area);
        frame.render_widget(inner_top_block_tweet_body, inner_top_block);

        /*
        3 here means - 1 for border L, 1 cursor, 1 border R
        MAX(3) because if area is < 3 we dont want to underflow
        we always extract 3 because that's the reserved space
        */
        let input_width = area.width.max(3) - 3;
        let scroll = self.input.visual_scroll(input_width as usize);
        let bottom_block = Paragraph::new(self.input.value())
            .scroll((0, scroll as u16))
            .block(Block::bordered().fg(PINK));

        /*
        `scroll` variable determines how much we need to scroll in respect to the whole width.

        Example:
        - Visible width: 17
        - Cursor is at column 21
        - `scroll` becomes 5, meaning the first 5 characters are hidden and
        drawing starts from character 5.

        in that case it's 5 (1 cause of the cursor)
        then, we need to shift cursor by `cursor_offset_x`
        The cursor position returned by `visual_cursor()` is still relative to the
        entire input, so we convert it into a position within the visible window by
        subtracting `scroll`.

        `cursor_offset_x = visual_cursor() - scroll`

        `max(scroll)` is a defensive guard against unsigned integer underflow in the
        unlikely event that `visual_cursor()` is less than `scroll`. In that case,
        the cursor is clamped to the left edge of the visible window.
         */
        let cursor_offset_x = self.input.visual_cursor().max(scroll) - scroll;
        frame.render_widget(bottom_block, input_area);
        frame.set_cursor_position(Position::new(
            input_area.x + cursor_offset_x as u16 + 1,
            input_area.y + 1,
        ));

        Ok(())
    }
}
