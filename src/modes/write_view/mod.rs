use crate::{
    app_state::AppState,
    modes::view::View,
    ui::{BaseElement, BottomBarElement, DividerElement, HeaderElement},
};

mod ui;
use ui::{BodyElement, StatusBarElement};

pub struct WriteView {
    elements: [Box<dyn BaseElement>; 5],
}

impl WriteView {
    pub fn new(app_state: &AppState) -> Self {
        Self {
            elements: [
                Box::new(HeaderElement::new(&app_state.user_info.username)),
                Box::new(DividerElement::default()),
                Box::new(BodyElement::new()),
                Box::new(StatusBarElement::new()),
                Box::new(BottomBarElement::default()),
            ],
        }
    }
}

impl View for WriteView {
    fn elements(&mut self) -> &mut [Box<dyn BaseElement>; 5] {
        &mut self.elements
    }
}
