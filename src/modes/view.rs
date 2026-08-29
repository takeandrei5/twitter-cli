use crossterm::event::{Event, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
};

use crate::{
    state::{Action, ElementState},
    ui::BaseElement,
    utils::ApplicationError,
};

pub trait View<T>
where
    T: ElementState,
{
    fn elements(&mut self) -> &mut [Box<dyn BaseElement<T>>; 5];

    fn render_view(&mut self, frame: &mut Frame, state: &T) -> Result<(), ApplicationError> {
        let layout = Layout::vertical([
            Constraint::Length(1), // header
            Constraint::Length(1), // divider
            Constraint::Min(0),    // tweet list
            Constraint::Length(1), // status bar
            Constraint::Length(2), // bottom bar
        ]);
        let [
            header_area,
            divider_area,
            body_area,
            status_bar_area,
            bottom_bar_area,
        ] = frame.area().layout(&layout);

        let [header, divider, body, status_bar, bottom_bar] = self.elements();

        header.draw(frame, header_area, state)?;
        divider.draw(frame, divider_area, state)?;
        body.draw(frame, body_area, state)?;
        status_bar.draw(frame, status_bar_area, state)?;
        bottom_bar.draw(frame, bottom_bar_area, state)?;

        Ok(())
    }

    async fn handle_key_event(
        &mut self,
        key: KeyEvent,
        event: &Event,
        state: &T,
    ) -> Result<Option<Action>, ApplicationError> {
        for element in self.elements() {
            if let Some(action) = element.handle_key_event(key, event, state).await? {
                return Ok(Some(action));
            }
        }

        Ok(None)
    }

    fn reset(&mut self) {
        self.elements().iter_mut().for_each(|x| x.reset());
    }
}
