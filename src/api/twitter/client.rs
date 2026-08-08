use std::collections::{HashMap, HashSet};

use chrono::{Duration, Utc};
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;

use super::models::{
    ApiUser, CreatePostResponse, CurrentUserResponse, IdPage, QuotePostRequest, TimelineResponse,
    Tweet, TweetIdRequest, UserInfo,
};
use crate::utils::ApplicationError;

pub struct TwitterClient {
    access_token: String,
    client: Client,
}

impl TwitterClient {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            client: Client::new(),
        }
    }

    pub async fn get_user_id(&self) -> Result<UserInfo, ApplicationError> {
        let request = self
            .client
            .get("https://api.x.com/2/users/me")
            .bearer_auth(&self.access_token);

        let response: CurrentUserResponse = self.send_json(request, "retrieve user info").await?;

        Ok(response.data)
    }

    pub async fn like_post(&self, tweet_id: &str, user_id: &str) -> Result<(), ApplicationError> {
        self.client
            .post(format!("https://api.x.com/2/users/{}/likes", user_id))
            .bearer_auth(&self.access_token)
            .json(&TweetIdRequest {
                tweet_id: tweet_id.to_owned(),
            })
            .send()
            .await
            .and_then(|response| response.error_for_status())
            .map_err(|err| {
                ApplicationError::XApiCallError(format!(
                    "Failed to like post {tweet_id}. Received error: {err}"
                ))
            })?;

        Ok(())
    }

    pub async fn unlike_post(&self, tweet_id: &str, user_id: &str) -> Result<(), ApplicationError> {
        self.client
            .delete(format!(
                "https://api.x.com/2/users/{}/likes/{}",
                user_id, tweet_id
            ))
            .bearer_auth(&self.access_token)
            .send()
            .await
            .and_then(|response| response.error_for_status())
            .map_err(|err| {
                ApplicationError::XApiCallError(format!(
                    "Failed to unlike post {tweet_id}. Received error: {err}"
                ))
            })?;

        Ok(())
    }

    pub async fn quote_post(
        &self,
        text: &str,
        quoted_tweet_id: &str,
    ) -> Result<(), ApplicationError> {
        let request = self
            .client
            .post("https://api.x.com/2/tweets")
            .bearer_auth(&self.access_token)
            .json(&QuotePostRequest {
                text: text.to_owned(),
                quote_tweet_id: quoted_tweet_id.to_owned(),
            });

        let _: CreatePostResponse = self.send_json(request, "create quote post").await?;

        Ok(())
    }

    pub async fn fetch_posts(&self, user_id: &str) -> Result<Vec<Tweet>, ApplicationError> {
        let (timeline, liked_ids) = tokio::try_join!(
            self.fetch_timeline(user_id),
            self.fetch_liked_post_ids(user_id)
        )?;

        let users: HashMap<String, ApiUser> = timeline
            .includes
            .users
            .into_iter()
            .map(|user| (user.id.clone(), user))
            .collect();

        timeline
            .data
            .into_iter()
            .map(|post| {
                let user = users.get(&post.author_id).ok_or_else(|| {
                    ApplicationError::XApiCallError(format!(
                        "The API did not return author {} for post {}",
                        post.author_id, post.id
                    ))
                })?;

                Ok(Tweet {
                    id: post.id.clone(),
                    name: user.name.clone(),
                    handle: format!("@{}", user.username),
                    time: post.created_at.naive_utc(),
                    body: post.text,
                    replies: post.public_metrics.reply_count,
                    retweets: post.public_metrics.retweet_count,
                    likes: post.public_metrics.like_count,
                    liked: liked_ids.contains(&post.id),
                    retweeted: false,
                })
            })
            .collect()
    }

    async fn fetch_timeline(&self, user_id: &str) -> Result<TimelineResponse, ApplicationError> {
        let today_start = Utc::now()
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .expect("midnight is always a valid time")
            .and_utc();
        let start_time = (today_start - Duration::days(1)).to_rfc3339();
        let end_time = (today_start + Duration::days(1)).to_rfc3339();

        let mut timeline = TimelineResponse::default();
        let mut next_token = None;

        loop {
            let mut request = self
                .client
                .get(format!(
                    "https://api.x.com/2/users/{}/timelines/reverse_chronological",
                    user_id
                ))
                .bearer_auth(&self.access_token)
                .query(&[
                    ("max_results", "100"),
                    ("start_time", start_time.as_str()),
                    ("end_time", end_time.as_str()),
                    ("exclude", "retweets"),
                    ("tweet.fields", "created_at,author_id,public_metrics"),
                    ("expansions", "author_id"),
                    ("user.fields", "name,username"),
                ]);

            if let Some(token) = next_token.as_deref() {
                request = request.query(&[("pagination_token", token)]);
            }

            let page = self
                .send_json::<TimelineResponse>(request, "fetch home timeline")
                .await?;
            timeline.data.extend(page.data);
            timeline.includes.users.extend(page.includes.users);

            match page.meta.next_token {
                Some(token) => next_token = Some(token),
                None => break,
            }
        }

        Ok(timeline)
    }

    async fn fetch_liked_post_ids(
        &self,
        user_id: &str,
    ) -> Result<HashSet<String>, ApplicationError> {
        let mut liked_ids = HashSet::new();
        let mut next_token = None;

        loop {
            let mut request = self
                .client
                .get(format!(
                    "https://api.x.com/2/users/{}/liked_tweets",
                    user_id
                ))
                .bearer_auth(&self.access_token)
                .query(&[("max_results", "100")]);

            if let Some(token) = next_token.as_deref() {
                request = request.query(&[("pagination_token", token)]);
            }

            let page = self
                .send_json::<IdPage>(request, "fetch liked posts")
                .await?;
            liked_ids.extend(page.data.into_iter().map(|post| post.id));

            match page.meta.next_token {
                Some(token) => next_token = Some(token),
                None => break,
            }
        }

        Ok(liked_ids)
    }

    async fn send_json<T>(
        &self,
        request: RequestBuilder,
        action: &str,
    ) -> Result<T, ApplicationError>
    where
        T: DeserializeOwned,
    {
        let map_err_action = |err: reqwest::Error| {
            ApplicationError::XApiCallError(format!("Failed to {action}. Received error: {err}"))
        };

        let response = request
            .send()
            .await
            .and_then(|response| response.error_for_status())
            .map_err(map_err_action)?;

        response.json::<T>().await.map_err(map_err_action)
    }

    pub async fn repost_post(&self, tweet_id: &str, user_id: &str) -> Result<(), ApplicationError> {
        self.client
            .post(format!("https://api.x.com/2/users/{}/retweets", user_id))
            .bearer_auth(&self.access_token)
            .json(&TweetIdRequest {
                tweet_id: tweet_id.to_owned(),
            })
            .send()
            .await
            .and_then(|response| response.error_for_status())
            .map_err(|err| {
                ApplicationError::XApiCallError(format!(
                    "Failed to repost post {tweet_id}. Received error: {err}"
                ))
            })?;

        Ok(())
    }
}
