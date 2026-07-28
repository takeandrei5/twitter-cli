use crossterm::event::{Event, KeyEvent};
use ratatui::{Frame, layout::Rect};

use crate::{app_state::AppState, utils::ApplicationError};

pub trait BaseElement {
    fn draw(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        app_state: &AppState,
    ) -> Result<(), ApplicationError>;

    fn handle_key_event(&mut self, _key: KeyEvent, _event: &Event, _app_state: &AppState) {}

    fn on_state_change(&mut self, _app_state: &AppState) {}
}
