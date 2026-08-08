use async_trait::async_trait;
use crossterm::event::{Event, KeyEvent};
use ratatui::{Frame, layout::Rect};

use crate::{
    app_state::{AppState, ReplyTarget},
    utils::ApplicationError,
};

#[async_trait(?Send)]
pub trait BaseElement {
    fn draw(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        app_state: &AppState,
    ) -> Result<(), ApplicationError>;

    fn handle_prepare_for_state_change(&mut self, _app_state: &AppState) -> Option<ReplyTarget> {
        None
    }

    async fn handle_key_event(
        &mut self,
        _key: KeyEvent,
        _event: &Event,
        _app_state: &mut AppState,
    ) -> Result<(), ApplicationError> {
        Ok(())
    }

    fn handle_state_change(&mut self, _app_state: &AppState) {}

    fn reset(&mut self) {}
}
