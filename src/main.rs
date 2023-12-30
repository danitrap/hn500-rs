#![deny(clippy::all)]

mod broadcast;
mod client;
mod config;
mod models;
mod utils;

use broadcast::send_telegram_message;
use client::fetch_hacker_news;
use config::Config;
use models::{HackerNews, HnItem};
use rss::Channel;
use simple_logger::SimpleLogger;
use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();
    let mut interval = interval(Duration::from_secs(60 * 10));
    let mut hacker_news = HackerNews::new();

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

        let content = setch_hacker_news().await;
        if content.is_err() {
            log::error!("Error fetching Hacker News: {:?}", content);
            continue;
        }

        let channel = Channel::read_from(&content.unwrap()[..]);
        if channel.is_err() {
            log::error!("Error parsing RSS: {:?}", channel);
            continue;
        }

        let items = channel
            .unwrap()
            .items()
            .iter()
            .filter_map(
                |item| match (item.title(), item.description(), item.guid()) {
                    (Some(title), Some(description), Some(guid)) => Some(HnItem::new(
                        title.to_string(),
                        description.to_string(),
                        guid.value.to_string(),
                    )),
                    _ => None,
                },
            )
            .collect::<Vec<_>>();

        let new_items = hacker_news.whats_new(items);

        match new_items {
            None => {
                log::info!("No new items");
                continue;
            }
            Some(items) => {
                log::info!("Sending {} new items to Telegram", items.len());
                for item in items {
                    let message = format!("{}", item);
                    send_telegram_message(&config, message).await;
                }
            }
        }
    }
}
