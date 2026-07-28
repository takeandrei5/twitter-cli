use crossterm::event::{Event, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
};

use crate::{
    app_state::AppState,
    modes::{view::View, write_view::ui::BodyElement},
    ui::{BaseElement, BottomBarElement, DividerElement, HeaderElement},
    utils::ApplicationError,
};

mod ui;
pub use ui::StatusBarElement;

pub struct WriteView {
    elements: [Box<dyn BaseElement>; 5],
}

impl WriteView {
    pub fn new() -> Self {
        Self {
            elements: [
                Box::new(HeaderElement::new()),
                Box::new(DividerElement::new()),
                Box::new(BodyElement::new()),
                Box::new(StatusBarElement::new()),
                Box::new(BottomBarElement::new()),
            ],
        }
    }
}

impl View for WriteView {
    fn render_view(
        &mut self,
        frame: &mut Frame,
        app_state: &AppState,
    ) -> Result<(), ApplicationError> {
        let layout = Layout::vertical([
            Constraint::Length(1), // header
            Constraint::Length(1), // divider
            Constraint::Min(0),    // tweet
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

        let [header, divider, body, status_bar, bottom_bar] = &mut self.elements;

        header.draw(frame, header_area, app_state)?;
        divider.draw(frame, divider_area, app_state)?;
        body.draw(frame, body_area, app_state)?;
        status_bar.draw(frame, status_bar_area, app_state)?;
        bottom_bar.draw(frame, bottom_bar_area, app_state)?;

        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent, event: &Event, app_state: &AppState) {
        self.elements
            .iter_mut()
            .for_each(|x| x.handle_key_event(key, event, app_state));
    }

    fn on_state_change(&mut self, app_state: &AppState) {
        self.elements
            .iter_mut()
            .for_each(|x| x.on_state_change(app_state));
    }
}
