#![deny(clippy::all)]

mod broadcast;
mod client;
mod config;
mod models;
mod utils;

use broadcast::send_telegram_message;
use client::fetch_hacker_news;
use config::Config;
use models::{ApplicationError, ApplicationError::*, HackerNews, HnItem};
use rss::Channel;
use simple_logger::SimpleLogger;
use tokio::time::{interval, Duration};

impl TryFrom<&rss::Item> for HnItem {
    type Error = &'static str;

    fn try_from(item: &rss::Item) -> Result<Self, Self::Error> {
        let title = item.title().ok_or("Missing title")?;
        let description = item.description().ok_or("Missing description")?;
        let guid = item.guid().ok_or("Missing guid")?.value().to_owned();

        Ok(Self::new(title, description, &guid))
    }
}

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();
    let mut interval = interval(Duration::from_secs(60 * 10));
    let mut hacker_news = HackerNews::new();
    let mut first_run = true;

    let config = match Config::new() {
        Ok(config) => config,
        Err(e) => {
            log::error!("Error loading config: {}", e);
            return;
        }
    };

    loop {
        interval.tick().await;

        log::info!("Fetching Hacker News");

        let items: Result<_, ApplicationError> = fetch_hacker_news()
            .await
            .map_err(|_| Fetching)
            .and_then(|res| Channel::read_from(&res[..]).map_err(|_| Parsing))
            .map(|channel| {
                channel
                    .items()
                    .iter()
                    .filter_map(|item| HnItem::try_from(item).ok())
                    .collect::<Vec<_>>()
            })
            .and_then(|items| hacker_news.whats_new(items).ok_or(NoNewItems))
            .and_then(|items| {
                if first_run {
                    Err(SkippingFirstRun)
                } else {
                    Ok(items)
                }
            });

        match items {
            Err(Fetching) => log::error!("Error fetching Hacker News"),
            Err(Parsing) => log::error!("Error parsing RSS"),
            Err(SkippingFirstRun) => {
                log::info!("Skipping first run");
                first_run = false;
            }
            Err(NoNewItems) => log::info!("No new items"),
            Ok(items) => {
                log::info!("Sending {} new items to Telegram", items.len());
                for item in items {
                    let message = format!("{}", item);
                    send_telegram_message(&config, message).await;
                }
            }
        };
    }
}
