use crate::api::Tweet;
use crate::state::StatusMessage;

#[derive(Clone, Debug)]
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

    pub fn tweet_by_id_mut(&mut self, tweet_id: &str) -> Option<&mut Tweet> {
        self.tweets.iter_mut().find(|tweet| tweet.id == tweet_id)
    }

    pub fn status_message(&self) -> Option<&StatusMessage> {
        self.status_message.as_ref()
    }

    pub(crate) fn set_status_message(&mut self, message: Option<StatusMessage>) {
        self.status_message = message;
    }
}
