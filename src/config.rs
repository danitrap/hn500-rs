//! This module is responsible for loading the configuration from the environment.

#![deny(clippy::all)]

#[cfg(feature = "dotenv")]
extern crate dotenv;

#[cfg(feature = "dotenv")]
use dotenv::dotenv;

use std::env;
use std::error::Error;

/// Holds the configuration for the application.
pub struct Config {
    /// The Telegram bot token.
    pub bot_token: String,
    /// The Telegram chat ID.
    pub chat_id: String,
}

impl Config {
    /// Creates a new `Config` instance from environment variables.
    pub fn new() -> Result<Self, Box<dyn Error>> {
        #[cfg(feature = "dotenv")]
        dotenv().ok();
        let bot_token = env::var("TELEGRAM_BOT_TOKEN")?;
        let chat_id = env::var("TELEGRAM_CHAT_ID")?;
        let config = Config { bot_token, chat_id };
        Ok(config)
    }
}
