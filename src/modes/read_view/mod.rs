use crate::{
    modes::view::View,
    state::{Mode, ReadState},
    ui::{BaseElement, BottomBarElement, DividerElement, HeaderElement},
};

mod ui;
use ui::{BodyElement, StatusBarElement};

pub struct ReadView {
    elements: [Box<dyn BaseElement<ReadState>>; 5],
}

impl ReadView {
    pub fn new(username: &str) -> Self {
        Self {
            elements: [
                Box::new(HeaderElement::new(username)),
                Box::new(DividerElement::new(Mode::Read)),
                Box::new(BodyElement::default()),
                Box::new(StatusBarElement),
                Box::new(BottomBarElement::for_mode(Mode::Read)),
            ],
        }
    }
}

impl View<ReadState> for ReadView {
    fn elements(&mut self) -> &mut [Box<dyn BaseElement<ReadState>>; 5] {
        &mut self.elements
    }
}
