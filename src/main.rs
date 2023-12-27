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
use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() {
    let mut interval = interval(Duration::from_secs(60 * 10));
    let mut hacker_news = HackerNews::new();
    let mut first_run = true;

    let config = match Config::new() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            return;
        }
    };

    loop {
        interval.tick().await;

        println!("Fetching Hacker News");

        let content = fetch_hacker_news().await;
        if content.is_err() {
            eprintln!("Error fetching Hacker News: {:?}", content);
            continue;
        }

        let channel = Channel::read_from(&content.unwrap()[..]);
        if channel.is_err() {
            eprintln!("Error parsing Hacker News: {:?}", channel);
            continue;
        }

        channel.unwrap().items().iter().for_each(|item| {
            if let (Some(title), Some(description)) = (item.title(), item.description()) {
                let hn_item = HnItem::new(title.to_string(), description.to_string());

                hacker_news.add_item(hn_item)
            }
        });

        let new_items = hacker_news.get_new_items();

        if first_run {
            first_run = false;
            continue;
        }

        for item in new_items {
            let message = format!("{}", item);
            send_telegram_message(&config, message).await;
        }
    }
}
