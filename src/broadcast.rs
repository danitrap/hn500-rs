use serde_json::json;
use std::env;

pub async fn send_telegram_message(message: String) {
    let bot_token = env::var("BOT_TOKEN");
    let chat_id = env::var("CHAT_ID");

    match (bot_token, chat_id) {
        (Ok(bot_token), Ok(chat_id)) => {
            let client = reqwest::Client::new();
            let _res = client
                .post(&format!(
                    "https://api.telegram.org/bot{}/sendMessage",
                    bot_token
                ))
                .json(&json!({
                    "chat_id": chat_id,
                    "text": message,
                }))
                .send()
                .await;
        }
        _ => {
            eprintln!("BOT_TOKEN or CHAT_ID not found in .env");
        }
    }
}
