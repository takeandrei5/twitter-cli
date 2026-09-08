use crate::api::Tweet;
use crate::state::StatusMessage;

#[derive(Clone, Debug, PartialEq)]
pub struct ReadState {
    status_message: Option<StatusMessage>,
    tweets: Vec<Tweet>,
}

impl ReadState {
    pub fn new(tweets: Vec<Tweet>) -> Self {
        Self {
            status_message: None,
            tweets,
        }
    }

    pub fn tweets(&self) -> &[Tweet] {
        &self.tweets
    }

    pub fn tweet_count(&self) -> usize {
        self.tweets.len()
    }

    pub fn tweet(&self, index: usize) -> Option<&Tweet> {
        self.tweets.get(index)
    }

    pub(crate) fn tweet_by_id_mut(&mut self, tweet_id: &str) -> Option<&mut Tweet> {
        self.tweets.iter_mut().find(|tweet| tweet.id == tweet_id)
    }

    pub fn status_message(&self) -> Option<&StatusMessage> {
        self.status_message.as_ref()
    }

    pub(crate) fn set_status_message(&mut self, message: Option<StatusMessage>) {
        self.status_message = message;
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn tweets() -> Vec<Tweet> {
        vec![
            Tweet {
                id: "test_id1".to_owned(),
                name: "test_name1".to_owned(),
                handle: "test_handle1".to_owned(),
                time: NaiveDateTime::default(),
                body: "test_body1".to_owned(),
                replies: 100,
                retweets: 100,
                likes: 100,
                liked: true,
                retweeted: false,
            },
            Tweet {
                id: "test_id2".to_owned(),
                name: "test_name2".to_owned(),
                handle: "test_handle2".to_owned(),
                time: NaiveDateTime::default(),
                body: "test_body2".to_owned(),
                replies: 100,
                retweets: 100,
                likes: 100,
                liked: true,
                retweeted: false,
            },
            Tweet {
                id: "test_id3".to_owned(),
                name: "test_name3".to_owned(),
                handle: "test_handle3".to_owned(),
                time: NaiveDateTime::default(),
                body: "test_body3".to_owned(),
                replies: 100,
                retweets: 100,
                likes: 100,
                liked: true,
                retweeted: false,
            },
        ]
    }

    #[rstest]
    fn tweets_should_return_correct_result(tweets: Vec<Tweet>) {
        // Arrange
        let sut = ReadState::new(tweets.clone());

        // Act
        let result = sut.tweets();

        // Assert
        assert_eq!(result, tweets);
    }

    #[rstest]
    fn tweet_count_should_return_correct_count(tweets: Vec<Tweet>) {
        // Arrange
        let sut = ReadState::new(tweets.clone());

        // Act
        let result = sut.tweet_count();

        // Assert
        assert_eq!(result, 3);
    }

    #[rstest]
    #[case(0, true)]
    #[case(1, true)]
    #[case(2, true)]
    #[case(3, false)]
    #[case(100, false)]
    fn tweet_should_return_some_for_valid_index_else_none(
        #[case] index: usize,
        #[case] has_data: bool,
        tweets: Vec<Tweet>,
    ) {
        // Arrange
        let sut = ReadState::new(tweets.clone());

        // Act
        let result = sut.tweet(index);

        // Assert
        assert_eq!(result.is_some(), has_data)
    }

    #[rstest]
    fn tweet_should_return_correct_data_for_valid_index(tweets: Vec<Tweet>) {
        // Arrange
        let sut = ReadState::new(tweets.clone());
        let expected_result = tweets.get(1).expect("index should be valid");

        // Act
        let result = sut.tweet(1).expect("index should be valid");

        // Assert
        assert_eq!(result, expected_result)
    }

    #[rstest]
    #[case("test_id1", true)]
    #[case("test_id2", true)]
    #[case("test_id3", true)]
    #[case("i am an invalid id", false)]
    fn tweet_by_id_mut_should_return_some_for_valid_it_else_none(
        #[case] id: &str,
        #[case] has_data: bool,
        tweets: Vec<Tweet>,
    ) {
        // Arrange
        let mut sut = ReadState::new(tweets.clone());

        // Act
        let result = sut.tweet_by_id_mut(id);

        // Assert
        assert_eq!(result.is_some(), has_data)
    }

    #[rstest]
    fn tweet_by_id_mut_should_correct_data_for_valid_id(tweets: Vec<Tweet>) {
        // Arrange
        let id = "test_id1";
        let expected_result = tweets.first().expect("index should be valid");
        let mut sut = ReadState::new(tweets.clone());

        // Act
        let result = sut.tweet_by_id_mut(id).expect("tweet id should be valid");

        // Assert
        assert_eq!(result, expected_result)
    }

    #[rstest]
    #[case(Some(StatusMessage::Liked))]
    #[case(None)]
    fn status_message_should_be_set_and_retrieved_correctly(
        #[case] status_message: Option<StatusMessage>,
        tweets: Vec<Tweet>,
    ) {
        // Arrange
        let mut sut = ReadState::new(tweets);
        sut.set_status_message(status_message.clone());

        let expected_result = status_message.as_ref();

        // Act
        let result = sut.status_message();

        // Assert
        assert_eq!(result, expected_result)
    }
}
