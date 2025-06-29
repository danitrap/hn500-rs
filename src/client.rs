//! This module is responsible for fetching the top 500 stories from Hacker News.

#![deny(clippy::all)]

use bytes::Bytes;

const HN_500_URL: &str = "https://hnrss.org/newest?points=500";

/// Fetches the top 500 stories from Hacker News.
pub async fn fetch_hacker_news() -> Result<Bytes, reqwest::Error> {
    let client = reqwest::Client::new();

    let res = client.get(HN_500_URL).send().await?.bytes().await?;

    Ok(res)
}
