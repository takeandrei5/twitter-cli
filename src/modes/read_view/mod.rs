use crate::{
    app_state::AppState,
    modes::view::View,
    ui::{BaseElement, BottomBarElement, DividerElement, HeaderElement},
};

mod ui;
use ui::{BodyElement, StatusBarElement};

pub struct ReadView {
    elements: [Box<dyn BaseElement>; 5],
}

impl ReadView {
    pub fn new(app_state: &AppState) -> Self {
        Self {
            elements: [
                Box::new(HeaderElement::new(&app_state.user_info.username)),
                Box::new(DividerElement),
                Box::new(BodyElement::default()),
                Box::new(StatusBarElement),
                Box::new(BottomBarElement::default()),
            ],
        }
    }
}

impl View for ReadView {
    fn elements(&mut self) -> &mut [Box<dyn BaseElement>; 5] {
        &mut self.elements
    }
}
