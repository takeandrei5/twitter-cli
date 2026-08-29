mod auth;
mod local;
mod twitter;

pub use auth::{AuthClient, TwitterConfig};
pub use local::LocalClient;
pub use twitter::open_tweet;
pub use twitter::{CreatePostOptions, Tweet, TwitterClient, UserInfo};
