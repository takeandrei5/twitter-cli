use crate::custom_widgets::Tweet;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Mode {
    Read,
    Write,
}

pub struct AppState {
    pub mode: Mode,
    pub tweet_count: usize,
    pub user_tag: Option<String>,
    pub reply_to_user_tweet: Option<Tweet>,
}

impl AppState {
    pub fn new(tweet_count: usize) -> Self {
        Self {
            mode: Mode::Write,
            tweet_count,
            user_tag: Some(String::from("cool_peanut")),
            reply_to_user_tweet: Some(
                Tweet {
            name: "Manish Goregaokar".into(),
            handle: "@ManishEarth".into(),
            time: chrono::Local::now().naive_local() - chrono::Duration::days(5),
            body: "Unicode is solved. Yes there are edge cases. Yes it handles them. No, rolling your own encoding is not the answer. Unicode is solved. Yes there are edge cases. Yes it handles them. No, rolling your own encoding is not the answer. Unicode is solved. Yes there are edge cases. Yes it handles them. No, rolling your own encoding is not the answer. Unicode is solved. Yes there are edge cases. Yes it handles them. No, rolling your own encoding is not the answer. Unicode is solved. Yes there are edge cases. Yes it handles them. No, rolling your own encoding is not the answer. Unicode is solved. Yes there are edge cases. Yes it handles them. No, rolling your own encoding is not the answer. Unicode is solved. Yes there are edge cases. Yes it handles them. No, rolling your own encoding is not the answer. Unicode is solved. Yes there are edge cases. Yes it handles them. No, rolling your own encoding is not the answer. Unicode is solved. Yes there are edge cases. Yes it handles them. No, rolling your own encoding is not the answer. Unicode is solved. Yes there are edge cases. Yes it handles them. No, rolling your own encoding is not the answer. Unicode is solved. Yes there are edge cases. Yes it handles them. No, rolling your own encoding is not the answer. Unicode is solved. Yes there are edge cases. Yes it handles them. No, rolling your own encoding is not the answer. Unicode is solved. Yes there are edge cases. Yes it handles them. No, rolling your own encoding is not the answer. Unicode is solved. Yes there are edge cases. Yes it handles them. No, rolling your own encoding is not the answer. Unicode is solved. Yes there are edge cases. Yes it handles them. No, rolling your own encoding is not the answer. Unicode is solved. Yes there are edge cases. Yes it handles them. No, rolling your own encoding is not the answer. Unicode is solved. Yes there are edge cases. Yes it handles them. No, rolling your own encoding is not the answer. Unicode is solved. Yes there are edge cases. Yes it handles them. No, rolling your own encoding is not the answer. Unicode is solved. Yes there are edge cases. Yes it handles them. No, rolling your own encoding is not the answer. Unicode is solved. Yes there are edge cases. Yes it handles them. No, rolling your own encoding is not the answer. Unicode is solved. Yes there are edge cases. Yes it handles them. No, rolling your own encoding is not the answer. Unicode is solved. Yes there are edge cases. Yes it handles them. No, rolling your own encoding is not the answer. Unicode is solved. Yes there are edge cases. Yes it handles them. No, rolling your own encoding is not the answer. Unicode is solved. Yes there are edge cases. Yes it handles them. No, rolling your own encoding is not the answer. Unicode is solved. Yes there are edge cases. Yes it handles them. No, rolling your own encoding is not the answer. Unicode is solved. Yes there are edge cases. Yes it handles them. No, rolling your own encoding is not the answer. Unicode is solved. Yes there are edge cases. Yes it handles them. No, rolling your own encoding is not the answer. Unicode is solved. Yes there are edge cases. Yes it handles them. No, rolling your own encoding is not the answer.!!!!".into(),
            replies: 178,
            retweets: 850,
            likes: 7100,
            liked: true,
            retweeted: false,
        })
        }
    }
}
