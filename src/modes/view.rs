use crossterm::event::{Event, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
};

use crate::{
    app_state::{AppState, ReplyTarget},
    ui::BaseElement,
    utils::ApplicationError,
};

pub trait View {
    fn elements(&mut self) -> &mut [Box<dyn BaseElement>; 5];

    fn render_view(
        &mut self,
        frame: &mut Frame,
        app_state: &AppState,
    ) -> Result<(), ApplicationError> {
        let layout = Layout::vertical([
            Constraint::Length(1), // header
            Constraint::Length(1), // divider
            Constraint::Min(0),    // tweet list
            Constraint::Length(1), // status bar
            Constraint::Length(1), // bottom bar
        ]);
        let [
            header_area,
            divider_area,
            body_area,
            status_bar_area,
            bottom_bar_area,
        ] = frame.area().layout(&layout);

        let [header, divider, body, status_bar, bottom_bar] = self.elements();

        header.draw(frame, header_area, app_state)?;
        divider.draw(frame, divider_area, app_state)?;
        body.draw(frame, body_area, app_state)?;
        status_bar.draw(frame, status_bar_area, app_state)?;
        bottom_bar.draw(frame, bottom_bar_area, app_state)?;

        Ok(())
    }

    async fn handle_key_event(
        &mut self,
        key: KeyEvent,
        event: &Event,
        app_state: &mut AppState,
    ) -> Result<(), ApplicationError> {
        for element in self.elements() {
            element.handle_key_event(key, event, app_state).await?;
        }

        Ok(())
    }

    fn on_state_change(&mut self, app_state: &AppState) {
        self.elements()
            .iter_mut()
            .for_each(|x| x.handle_state_change(app_state));
    }

    fn prepare_for_state_change(&mut self, app_state: &AppState) -> Option<ReplyTarget> {
        self.elements()
            .iter_mut()
            .find_map(|x| x.handle_prepare_for_state_change(app_state))
    }

    fn reset(&mut self) {
        self.elements().iter_mut().for_each(|x| x.reset());
    }
}
