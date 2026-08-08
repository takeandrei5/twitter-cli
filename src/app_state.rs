use crate::{
    api::{Tweet, TwitterClient, UserInfo},
    utils::ApplicationError,
};

#[derive(Debug, Clone)]
pub struct ReplyTarget {
    pub tweet: Tweet,
    pub handle: String,
}

impl PartialEq for ReplyTarget {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle && self.tweet.id == other.tweet.id
    }
}

impl Eq for ReplyTarget {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode {
    Read,
    Write { reply_target: ReplyTarget },
}

pub enum ViewAction {
    SwitchToRead,
}

pub struct AppState {
    pub mode: Mode,
    pub twitter_client: TwitterClient,
    pub tweets: Vec<Tweet>,
    pub user_info: UserInfo,
}

impl AppState {
    pub async fn new(twitter_client: TwitterClient) -> Result<Self, ApplicationError> {
        let user_info = twitter_client.get_user_id().await?;
        let tweets = twitter_client.fetch_posts(&user_info.id).await?;

        Ok(Self {
            mode: Mode::Read, // must always start in read
            twitter_client,
            tweets,
            user_info,
        })
    }

    pub async fn refresh_tweets(&mut self) -> Result<(), ApplicationError> {
        let data = self.twitter_client.fetch_posts(&self.user_info.id).await?;

        self.tweets = data;

        Ok(())
    }
}
