extern crate dotenv;

use dotenv::dotenv;
use std::env;
use std::error::Error;

pub struct Config {
    pub bot_token: String,
    pub chat_id: String,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        dotenv().ok();
        let bot_token = env::var("TELEGRAM_BOT_TOKEN")?;
        let chat_id = env::var("TELEGRAM_CHAT_ID")?;
        let config = Config { bot_token, chat_id };
        Ok(config)
    }
}
