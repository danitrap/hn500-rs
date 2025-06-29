//! This module is responsible for sending messages to the Telegram chat.

#![deny(clippy::all)]

use crate::config::Config;
use serde_json::json;

/// Sends a message to the Telegram chat.
pub async fn send_telegram_message(config: &Config, message: &str) {
    let client = reqwest::Client::new();
    let _res = client
        .post(&format!(
            "https://api.telegram.org/bot{}/sendMessage",
            config.bot_token
        ))
        .json(&json!({
            "chat_id": config.chat_id,
            "text": message,
        }))
        .send()
        .await;
}
