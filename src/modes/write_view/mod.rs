use crate::{
    modes::view::View,
    state::{Mode, WriteState},
    ui::{BaseElement, BottomBarElement, DividerElement, HeaderElement},
};

mod ui;
use ui::{BodyElement, StatusBarElement};

pub struct WriteView {
    elements: [Box<dyn BaseElement<WriteState>>; 5],
}

impl WriteView {
    pub fn new(username: &str) -> Self {
        Self {
            elements: [
                Box::new(HeaderElement::new(username)),
                Box::new(DividerElement::new(Mode::Write)),
                Box::new(BodyElement::default()),
                Box::new(StatusBarElement),
                Box::new(BottomBarElement::new(Mode::Write)),
            ],
        }
    }
}

impl View<WriteState> for WriteView {
    fn elements(&mut self) -> &mut [Box<dyn BaseElement<WriteState>>; 5] {
        &mut self.elements
    }
}
