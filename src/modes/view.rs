use crossterm::event::{Event, KeyEvent};
use ratatui::Frame;

use crate::{app_state::AppState, utils::ApplicationError};

pub trait View {
    fn render_view(
        &mut self,
        frame: &mut Frame,
        app_state: &AppState,
    ) -> Result<(), ApplicationError>;

    fn handle_key_event(&mut self, _key: KeyEvent, _event: &Event, _app_state: &AppState);

    fn on_state_change(&mut self, _app_state: &AppState);
}
