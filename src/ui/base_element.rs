use async_trait::async_trait;
use crossterm::event::{Event, KeyEvent};
use ratatui::{Frame, layout::Rect};

use crate::{
    state::Action,
    state::{AppState, ElementState},
    utils::ApplicationError,
};

#[async_trait(?Send)]
pub trait BaseElement<T = AppState>
where
    T: ElementState,
{
    fn draw(&mut self, frame: &mut Frame, area: Rect, state: &T) -> Result<(), ApplicationError>;

    async fn handle_key_event(
        &mut self,
        _key: KeyEvent,
        _event: &Event,
        _state: &T,
    ) -> Result<Option<Action>, ApplicationError> {
        Ok(None)
    }

    fn reset(&mut self) {}
}
