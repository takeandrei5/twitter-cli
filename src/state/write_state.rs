use crate::state::StatusMessage;

#[derive(Clone, Default, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(Some(StatusMessage::Liked))]
    #[case(None)]
    fn status_message_should_be_set_and_retrieved_correctly(
        #[case] status_message: Option<StatusMessage>,
    ) {
        // Arrange
        let mut sut = WriteState::default();
        sut.set_status_message(status_message.clone());

        let expected_result = status_message.as_ref();

        // Act
        let result = sut.status_message();

        // Assert
        assert_eq!(result, expected_result)
    }
}
