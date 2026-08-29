use crate::api::{CreatePostOptions, Tweet};

#[derive(Debug, Clone, Copy)]
pub enum AppEvent {
    ResetReadState,
    ResetWriteState,
}

#[derive(Debug, PartialEq)]
pub enum Action {
    SwitchMode,
    RefreshTweets,
    Quit,
    LikeTweet {
        tweet_id: String,
        was_liked: bool,
    },
    RetweetTweet {
        tweet_id: String,
    },
    OpenTweet(Tweet),
    CreatePost {
        text: String,
        options: CreatePostOptions,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionOutcome {
    None,
    ResetReadView,
    ResetWriteView,
    DelayedResetView,
    Quit,
}
