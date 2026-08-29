use crate::state::StatusMessage;

#[derive(Clone, Default)]
pub struct WriteState {
    status_message: Option<StatusMessage>,
}

impl WriteState {
    pub fn status_message(&self) -> Option<&StatusMessage> {
        self.status_message.as_ref()
    }

    pub(crate) fn set_status_message(&mut self, message: Option<StatusMessage>) {
        self.status_message = message;
    }
}
