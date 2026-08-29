
use crate::state::{AppState, ReadState, WriteState};

pub trait ElementState {
    fn reset_status_message(&mut self) {}
}

impl ElementState for ReadState {
    fn reset_status_message(&mut self) {
        self.set_status_message(None);
    }
}

impl ElementState for AppState {}

impl ElementState for WriteState {
    fn reset_status_message(&mut self) {
        self.set_status_message(None);
    }
}
