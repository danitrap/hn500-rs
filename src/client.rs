use bytes::Bytes;

const HN_500_URL: &str = "https://hnrss.org/newest?points=500";

pub async fn fetch_hacker_news() -> Result<Bytes, reqwest::Error> {
    let client = reqwest::Client::new();

    let res = client.get(HN_500_URL).send().await?.bytes().await?;

    Ok(res)
}
