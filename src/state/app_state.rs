use crate::{
    api::{CreatePostOptions, TwitterClient, UserInfo, open_tweet},
    state::{Action, ActionOutcome, AppEvent, ElementState, ReadState, WriteState},
    utils::ApplicationError,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Read,
    Write,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Mode::Read => "read",
            Mode::Write => "write",
        };

        write!(f, "{}", text)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatusMessage {
    Liked,
    Unliked,
    Retweeted,
    AlreadyRetweeted,
    NewPostAdded,
}

pub struct AppState {
    mode: Mode,
    twitter_client: TwitterClient,
    user_info: UserInfo,
    read_state: ReadState,
    write_state: WriteState,
    event_sender: tokio::sync::mpsc::UnboundedSender<AppEvent>,
    event_receiver: tokio::sync::mpsc::UnboundedReceiver<AppEvent>,
}

impl AppState {
    pub async fn new(twitter_client: TwitterClient) -> Result<Self, ApplicationError> {
        let user_info = twitter_client.get_current_user().await?;

        let tweets = twitter_client.fetch_posts(&user_info.id).await?;
        let read_state = ReadState::new(tweets);
        let write_state = WriteState::default();
        let (event_sender, event_receiver) = tokio::sync::mpsc::unbounded_channel();

        Ok(Self {
            mode: Mode::Read, // must always start in read
            twitter_client,
            user_info,
            read_state,
            write_state,
            event_sender,
            event_receiver,
        })
    }

    pub fn mode(&self) -> &Mode {
        &self.mode
    }

    pub fn toggle_mode(&mut self) {
        let new_mode = match self.mode {
            Mode::Read => Mode::Write,
            Mode::Write => Mode::Read,
        };
        self.mode = new_mode;
    }

    pub fn user_id(&self) -> &str {
        &self.user_info.id
    }

    pub fn username(&self) -> &str {
        &self.user_info.username
    }

    pub async fn refresh_tweets(&mut self) -> Result<(), ApplicationError> {
        let tweets = self.twitter_client.fetch_posts(self.user_id()).await?;
        self.read_state = ReadState::new(tweets);
        Ok(())
    }

    pub fn show_status_message(&mut self, new_status_message: StatusMessage) {
        self.read_state
            .set_status_message(Some(new_status_message.clone()));
        self.write_state
            .set_status_message(Some(new_status_message));
    }

    pub fn process_events(&mut self) {
        while let Ok(event) = self.event_receiver.try_recv() {
            match event {
                AppEvent::ResetReadState => self.read_state.reset_status_message(),
                AppEvent::ResetWriteState => self.write_state.reset_status_message(),
            }
        }
    }

    pub fn event_sender(&self) -> tokio::sync::mpsc::UnboundedSender<AppEvent> {
        self.event_sender.clone()
    }

    pub fn read_state(&self) -> &ReadState {
        &self.read_state
    }

    pub fn write_state(&self) -> &WriteState {
        &self.write_state
    }

    pub async fn handle_action(
        &mut self,
        action: Action,
    ) -> Result<ActionOutcome, ApplicationError> {
        match action {
            Action::SwitchMode => {
                self.toggle_mode();
                return Ok(ActionOutcome::ResetWriteView);
            }
            Action::RefreshTweets => {
                self.refresh_tweets().await?;
                return Ok(ActionOutcome::ResetReadView);
            }
            Action::Quit => return Ok(ActionOutcome::Quit),
            Action::LikeTweet {
                tweet_id,
                was_liked,
            } => {
                self.handle_like_action(&tweet_id, was_liked).await?;

                return Ok(ActionOutcome::DelayedResetView);
            }
            Action::RetweetTweet { tweet_id } => {
                self.handle_retweet_action(&tweet_id).await?;

                return Ok(ActionOutcome::DelayedResetView);
            }
            Action::OpenTweet(tweet) => {
                if let Err(error) = open_tweet(&tweet) {
                    tracing::debug!(tweet_id = %tweet.id, error = %error, "Could not open tweet");
                }
            }
            Action::CreatePost { text, options } => {
                self.handle_create_post(text, options).await?;

                return Ok(ActionOutcome::DelayedResetView);
            }
        }

        Ok(ActionOutcome::None)
    }

    async fn handle_create_post(
        &mut self,
        text: String,
        options: CreatePostOptions,
    ) -> Result<(), ApplicationError> {
        self.twitter_client.create_post(&text, options).await?;
        self.show_status_message(StatusMessage::NewPostAdded);
        self.toggle_mode();

        Ok(())
    }

    async fn handle_like_action(
        &mut self,
        tweet_id: &str,
        was_liked: bool,
    ) -> Result<(), ApplicationError> {
        if was_liked {
            self.twitter_client
                .unlike_post(tweet_id, self.user_id())
                .await?;
        } else {
            self.twitter_client
                .like_post(tweet_id, self.user_id())
                .await?;
        }

        if let Some(tweet) = self.read_state.tweet_by_id_mut(tweet_id) {
            tweet.liked = !was_liked;
            tweet.likes = if was_liked {
                tweet.likes.saturating_sub(1)
            } else {
                tweet.likes.saturating_add(1)
            };
        }

        self.show_status_message(if was_liked {
            StatusMessage::Unliked
        } else {
            StatusMessage::Liked
        });

        Ok(())
    }

    async fn handle_retweet_action(&mut self, tweet_id: &str) -> Result<(), ApplicationError> {
        let already_retweeted = self
            .read_state
            .tweets()
            .iter()
            .find(|tweet| tweet.id == tweet_id)
            .is_some_and(|tweet| tweet.retweeted);

        if already_retweeted {
            self.show_status_message(StatusMessage::AlreadyRetweeted);
            return Ok(());
        }

        self.twitter_client
            .retweet_post(tweet_id, self.user_id())
            .await?;

        if let Some(tweet) = self.read_state.tweet_by_id_mut(tweet_id) {
            tweet.retweeted = true;
        }

        self.show_status_message(StatusMessage::Retweeted);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app_state() -> AppState {
        let (event_sender, event_receiver) = tokio::sync::mpsc::unbounded_channel();

        AppState {
            mode: Mode::Read,
            twitter_client: TwitterClient::new(String::from("test-token"), None),
            user_info: UserInfo {
                id: String::from("test-user-id"),
                username: String::from("test-user"),
            },
            read_state: ReadState::new(vec![]),
            write_state: WriteState::default(),
            event_sender,
            event_receiver,
        }
    }

    #[test]
    fn toggle_mode_should_switch_between_read_and_write() {
        // Arrange
        let mut sut = app_state();

        // Act
        sut.toggle_mode();
        let mode_after_first_toggle = *sut.mode();

        sut.toggle_mode();
        let mode_after_second_toggle = *sut.mode();

        // Assert
        assert_eq!(mode_after_first_toggle, Mode::Write);
        assert_eq!(mode_after_second_toggle, Mode::Read);
    }

    #[test]
    fn mode_should_return_the_expected_mode() {
        // Arrange
        let sut = app_state();

        // Act
        let mode = sut.mode();

        // Assert
        assert_eq!(mode, &Mode::Read);
    }

    #[test]
    fn user_id_should_return_the_expected_user_id() {
        // Arrange
        let sut = app_state();

        // Act
        let user_id = sut.user_id();

        // Assert
        assert_eq!(user_id, "test-user-id");
    }

    #[test]
    fn username_should_return_the_expected_username() {
        // Arrange
        let sut = app_state();

        // Act
        let username = sut.username();

        // Assert
        assert_eq!(username, "test-user");
    }

    #[test]
    fn read_state_should_return_the_expected_read_state() {
        // Arrange
        let sut = app_state();

        // Act
        let read_state = sut.read_state();

        // Assert
        assert_eq!(*read_state, sut.read_state);
    }

    #[test]
    fn write_state_should_return_the_expected_write_state() {
        // Arrange
        let sut = app_state();

        // Act
        let write_state = sut.write_state();

        // Assert
        assert_eq!(*write_state, sut.write_state);
    }

    #[test]
    fn show_status_message_should_update_read_and_write_states() {
        // Arrange
        let mut sut = app_state();

        // Act
        sut.show_status_message(StatusMessage::Liked);

        // Assert
        assert_eq!(sut.read_state.status_message(), Some(&StatusMessage::Liked));
        assert_eq!(sut.write_state.status_message(), Some(&StatusMessage::Liked));
    }
}
