//! This module is responsible for fetching new stories from Hacker News that have ≥500 points.

#![deny(clippy::all)]

use bytes::Bytes;

const HN_500_URL: &str = "https://hnrss.org/newest?points=500";

/// Fetches new stories from Hacker News that have ≥500 points.
pub async fn fetch_hacker_news(client: &reqwest::Client) -> Result<Bytes, reqwest::Error> {
    let res = client.get(HN_500_URL).send().await?.bytes().await?;

    Ok(res)
}
