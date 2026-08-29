use crate::{
    api::CreatePostOptions,
    state::{Action, WriteState},
    ui::BaseElement,
    utils::{ApplicationError, BG, BLUE, PINK, TEXT_DIM},
};
use async_trait::async_trait;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Position, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Paragraph},
};
use tui_input::{Input, backend::crossterm::EventHandler};

#[derive(Debug, Clone, Default)]
pub struct BodyElement {
    input: Input,
    options: CreatePostOptions,
    focused_option: OptionFocus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum OptionFocus {
    #[default]
    Input,
    ShareWithFollowers,
    PaidPartnership,
    Promoted,
}

impl BodyElement {
    fn focus_next(&mut self) {
        self.focused_option = match self.focused_option {
            OptionFocus::Input => OptionFocus::ShareWithFollowers,
            OptionFocus::ShareWithFollowers => OptionFocus::PaidPartnership,
            OptionFocus::PaidPartnership => OptionFocus::Promoted,
            OptionFocus::Promoted => OptionFocus::Input,
        };
    }

    fn focus_previous(&mut self) {
        self.focused_option = match self.focused_option {
            OptionFocus::Input => OptionFocus::Promoted,
            OptionFocus::ShareWithFollowers => OptionFocus::Input,
            OptionFocus::PaidPartnership => OptionFocus::ShareWithFollowers,
            OptionFocus::Promoted => OptionFocus::PaidPartnership,
        };
    }

    fn toggle_focused_option(&mut self) {
        match self.focused_option {
            OptionFocus::Input => {}
            OptionFocus::ShareWithFollowers => {
                self.options.share_with_followers = !self.options.share_with_followers;
            }
            OptionFocus::PaidPartnership => {
                self.options.paid_partnership = !self.options.paid_partnership;
            }
            OptionFocus::Promoted => {
                self.options.nullcast = !self.options.nullcast;
            }
        }
    }

    fn submit_action(&self) -> Action {
        Action::CreatePost {
            text: self.input.value().to_owned(),
            options: self.options,
        }
    }

    fn draw_options(&self, frame: &mut Frame, area: Rect) {
        let options_color = if self.focused_option == OptionFocus::Input {
            TEXT_DIM
        } else {
            PINK
        };
        let options_block = Block::bordered().title("Options").fg(options_color);
        let option_areas =
            Layout::horizontal([Constraint::Fill(1); 3]).split(options_block.inner(area));

        frame.render_widget(options_block, area);

        let options = [
            (
                OptionFocus::ShareWithFollowers,
                self.options.share_with_followers,
                "Share with followers",
            ),
            (
                OptionFocus::PaidPartnership,
                self.options.paid_partnership,
                "Paid partnership",
            ),
            (OptionFocus::Promoted, self.options.nullcast, "Promoted"),
        ];

        for ((focus, checked, label), option_area) in options.into_iter().zip(option_areas.iter()) {
            let checkbox = if checked { "[x]" } else { "[ ]" };
            let style = if self.focused_option == focus {
                Style::default().fg(BLUE).bold()
            } else {
                Style::default()
            };

            let line = Line::styled(format!(" {checkbox} {label}"), style);
            frame.render_widget(line, *option_area);
        }
    }
}

#[async_trait(?Send)]
impl BaseElement<WriteState> for BodyElement {
    async fn handle_key_event(
        &mut self,
        key: KeyEvent,
        event: &Event,
        _state: &WriteState,
    ) -> Result<Option<Action>, ApplicationError> {
        match (key.modifiers, key.code) {
            (KeyModifiers::NONE, KeyCode::Tab) | (KeyModifiers::SHIFT, KeyCode::BackTab) => {
                self.focus_next()
            }
            (KeyModifiers::SHIFT, KeyCode::Tab) | (KeyModifiers::NONE, KeyCode::BackTab) => {
                self.focus_previous()
            }
            (KeyModifiers::NONE, KeyCode::Char(' '))
                if self.focused_option != OptionFocus::Input =>
            {
                self.toggle_focused_option()
            }
            (KeyModifiers::CONTROL, KeyCode::Char('s')) => {
                return Ok(Some(self.submit_action()));
            }
            _ if self.focused_option == OptionFocus::Input => {
                self.input.handle_event(event);
            }
            _ => {}
        }

        Ok(None)
    }

    fn draw(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        _state: &WriteState,
    ) -> Result<(), ApplicationError> {
        let container = Block::default().bg(BG);
        frame.render_widget(container, area);

        let [input_area, options_area] = area.layout(&Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(3),
        ]));

        let input_width = input_area.width.max(3) - 3;
        let scroll = self.input.visual_scroll(input_width as usize);
        let input_color = if self.focused_option == OptionFocus::Input {
            PINK
        } else {
            TEXT_DIM
        };
        let bottom_block = Paragraph::new(self.input.value())
            .scroll((0, scroll as u16))
            .block(Block::bordered().title("New post").fg(input_color));

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
        if self.focused_option == OptionFocus::Input {
            frame.set_cursor_position(Position::new(
                input_area.x + cursor_offset_x as u16 + 1,
                input_area.y + 1,
            ));
        }
        self.draw_options(frame, options_area);

        Ok(())
    }

    fn reset(&mut self) {
        self.input.reset();
        self.options = CreatePostOptions::default();
        self.focused_option = OptionFocus::Input;
    }
}
