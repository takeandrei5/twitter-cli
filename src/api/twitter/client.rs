use std::collections::{HashMap, HashSet};

use chrono::{Duration, Utc};
use reqwest::{Client, RequestBuilder, Response, StatusCode};
use serde::de::DeserializeOwned;

use super::models::{
    ApiUser, CreatePostOptions, CreatePostRequest, CurrentUserResponse, IdPage, TimelineResponse,
    Tweet, TweetIdRequest, UserInfo,
};
use crate::utils::ApplicationError;

#[derive(Clone, Debug)]
pub struct TwitterClient {
    access_token: String,
    base_url: String,
    client: Client,
}

impl TwitterClient {
    pub fn new(access_token: String, alternative_base_url: Option<String>) -> Self {
        Self {
            access_token,
            base_url: alternative_base_url.unwrap_or(String::from("https://api.x.com")),
            client: Client::new(),
        }
    }

    pub async fn get_current_user(&self) -> Result<UserInfo, ApplicationError> {
        let request = self
            .client
            .get(format!("{}/2/users/me", self.base_url))
            .bearer_auth(&self.access_token);

        let response: CurrentUserResponse = self.send_json(request, "retrieve user info").await?;

        Ok(response.data)
    }

    pub async fn like_post(&self, tweet_id: &str, user_id: &str) -> Result<(), ApplicationError> {
        let request = self
            .client
            .post(format!("{}/2/users/{}/likes", self.base_url, user_id))
            .bearer_auth(&self.access_token)
            .json(&TweetIdRequest {
                tweet_id: tweet_id.to_owned(),
            });

        self.send_empty(request, "like post").await?;

        Ok(())
    }

    pub async fn unlike_post(&self, tweet_id: &str, user_id: &str) -> Result<(), ApplicationError> {
        let request = self
            .client
            .delete(format!(
                "{}/2/users/{}/likes/{}",
                self.base_url, user_id, tweet_id
            ))
            .bearer_auth(&self.access_token);

        self.send_empty(request, "unlike post").await?;

        Ok(())
    }

    pub async fn create_post(
        &self,
        text: &str,
        options: CreatePostOptions,
    ) -> Result<(), ApplicationError> {
        let request = self
            .client
            .post(format!("{}/2/tweets", self.base_url))
            .bearer_auth(&self.access_token)
            .json(&CreatePostRequest {
                text: text.to_owned(),
                options,
            });

        self.send_empty(request, "create post").await?;

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
                    ApplicationError::XApiUnexpectedError(format!(
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
                    "{}/2/users/{}/timelines/reverse_chronological",
                    self.base_url, user_id
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
                    "{}/2/users/{}/liked_tweets",
                    self.base_url, user_id
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
            ApplicationError::XApiUnexpectedError(format!(
                "Failed to {action}. Received error: {err}"
            ))
        };

        let response = Self::send_empty(self, request, action).await?;

        response.json::<T>().await.map_err(map_err_action)
    }

    async fn send_empty(
        &self,
        request: RequestBuilder,
        action: &str,
    ) -> Result<Response, ApplicationError> {
        let map_err_action = |err: reqwest::Error| {
            ApplicationError::XApiUnexpectedError(format!(
                "Failed to {action}. Received error: {err}"
            ))
        };

        let response = request.send().await.map_err(map_err_action)?;

        let status = response.status();
        if !status.is_success() {
            let response_text = response.text().await.map_err(map_err_action)?;

            return Err(match status {
                StatusCode::BAD_REQUEST => ApplicationError::XApiBadRequestError(response_text),
                StatusCode::UNAUTHORIZED => ApplicationError::XApiUnauthorizedError(response_text),
                StatusCode::FORBIDDEN => ApplicationError::XApiForbiddenError(response_text),
                StatusCode::NOT_FOUND => ApplicationError::XApiNotFoundError(response_text),
                _ => ApplicationError::XApiUnexpectedError(format!(
                    "Failed to {action}. X API returned HTTP {status}: {response_text}"
                )),
            });
        }

        Ok(response)
    }

    pub async fn retweet_post(
        &self,
        tweet_id: &str,
        user_id: &str,
    ) -> Result<(), ApplicationError> {
        self.client
            .post(format!("{}/2/users/{}/retweets", self.base_url, user_id))
            .bearer_auth(&self.access_token)
            .json(&TweetIdRequest {
                tweet_id: tweet_id.to_owned(),
            })
            .send()
            .await
            .and_then(|response| response.error_for_status())
            .map_err(|err| {
                ApplicationError::XApiUnexpectedError(format!(
                    "Failed to retweet post {tweet_id}. Received error: {err}"
                ))
            })?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;
    use serde_json::json;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{body_json, header, method, path},
    };

    use super::*;

    #[tokio::test]
    async fn like_post_should_send_correct_request() {
        // Arrange
        let mock_server = MockServer::start().await;
        let tweet_id = "2";
        let user_id = "1";

        Mock::given(method("POST"))
            .and(header("authorization", "Bearer test-token"))
            .and(path(format!("/2/users/{user_id}/likes")))
            .and(body_json(json!({
                "tweet_id": tweet_id
            })))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let sut = TwitterClient::new(String::from("test-token"), Some(mock_server.uri()));

        // Act
        let result = sut.like_post(tweet_id, user_id).await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn unlike_post_should_send_correct_request() {
        // Arrange
        let mock_server = MockServer::start().await;
        let tweet_id = "2";
        let user_id = "1";

        Mock::given(method("DELETE"))
            .and(header("authorization", "Bearer test-token"))
            .and(path(format!("/2/users/{user_id}/likes/{tweet_id}")))
            .respond_with(ResponseTemplate::new(204))
            .expect(1)
            .mount(&mock_server)
            .await;

        let sut = TwitterClient::new(String::from("test-token"), Some(mock_server.uri()));

        // Act
        let result = sut.unlike_post(tweet_id, user_id).await;

        // Assert
        assert!(result.is_ok());
    }

    #[rstest]
    #[case(true, true, true)]
    #[case(true, true, false)]
    #[case(true, false, true)]
    #[case(true, false, false)]
    #[case(false, true, true)]
    #[case(false, true, false)]
    #[case(false, false, true)]
    #[case(false, false, false)]
    #[tokio::test]
    async fn create_post_should_send_correct_request(
        #[case] share_with_followers: bool,
        #[case] paid_partnership: bool,
        #[case] nullcast: bool,
    ) {
        // Arrange
        let mock_server = MockServer::start().await;
        let options = CreatePostOptions {
            share_with_followers,
            paid_partnership,
            nullcast,
        };
        let text = "thisisatest";

        Mock::given(method("POST"))
            .and(header("authorization", "Bearer test-token"))
            .and(path("/2/tweets".to_string()))
            .and(body_json(json!({
                "text": text.to_owned(),
                "share_with_followers": share_with_followers,
                "paid_partnership": paid_partnership,
                "nullcast": nullcast
            })))
            .respond_with(ResponseTemplate::new(204))
            .expect(1)
            .mount(&mock_server)
            .await;

        let sut = TwitterClient::new(String::from("test-token"), Some(mock_server.uri()));

        // Act
        let result = sut.create_post(text, options).await;

        // Assert
        assert!(result.is_ok())
    }
}
