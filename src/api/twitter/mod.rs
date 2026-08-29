mod client;
mod models;

pub use client::TwitterClient;
pub use models::{CreatePostOptions, Tweet, UserInfo};

use crate::utils::ApplicationError;

pub fn open_tweet(tweet: &Tweet) -> Result<(), ApplicationError> {
    let handle = tweet.handle.trim_start_matches('@');
    let url = format!("https://x.com/{handle}/status/{}", tweet.id);

    webbrowser::open(&url).map_err(|error| {
        ApplicationError::UnexpectedError(std::io::Error::other(format!(
            "Could not open tweet URL {url}: {error}"
        )))
    })?;

    Ok(())
}
