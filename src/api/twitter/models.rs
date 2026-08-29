use chrono::{DateTime, FixedOffset, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct TweetIdRequest {
    pub tweet_id: String,
}

#[derive(Serialize)]
pub struct CreatePostRequest {
    pub text: String,
    #[serde(flatten)]
    pub options: CreatePostOptions,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub struct CreatePostOptions {
    pub share_with_followers: bool,
    pub paid_partnership: bool,
    pub nullcast: bool,
}

impl Default for CreatePostOptions {
    fn default() -> Self {
        Self {
            share_with_followers: true,
            paid_partnership: false,
            nullcast: false,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct CreatedPost {
    pub id: String,
    pub text: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct TimelineResponse {
    #[serde(default)]
    pub data: Vec<TimelinePost>,
    #[serde(default)]
    pub includes: TimelineIncludes,
    #[serde(default)]
    pub meta: PaginationMeta,
}

#[derive(Debug, Deserialize)]
pub struct TimelinePost {
    pub id: String,
    pub author_id: String,
    pub created_at: DateTime<FixedOffset>,
    pub text: String,
    pub public_metrics: PublicMetrics,
}

#[derive(Debug, Default, Deserialize)]
pub struct TimelineIncludes {
    #[serde(default)]
    pub users: Vec<ApiUser>,
}

#[derive(Debug, Deserialize)]
pub struct ApiUser {
    pub id: String,
    pub name: String,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct CurrentUserResponse {
    pub data: UserInfo,
}

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Tweet {
    pub id: String,
    pub name: String,
    pub handle: String,
    pub time: NaiveDateTime,
    pub body: String,
    pub replies: u64,
    pub retweets: u64,
    pub likes: u64,
    pub liked: bool,
    pub retweeted: bool,
}

#[derive(Debug, Deserialize)]
pub struct PublicMetrics {
    pub reply_count: u64,
    pub retweet_count: u64,
    pub like_count: u64,
}

#[derive(Debug, Deserialize)]
pub struct IdPage {
    #[serde(default)]
    pub data: Vec<PostId>,
    #[serde(default)]
    pub meta: PaginationMeta,
}

#[derive(Debug, Deserialize)]
pub struct PostId {
    pub id: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct PaginationMeta {
    pub next_token: Option<String>,
}
