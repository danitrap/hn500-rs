# Hacker News 500 Telegram Bot

This is a Rust program that fetches the top 500 stories from Hacker News and
sends them to a Telegram chat.

It deduplicates the stories and only sends the new ones.

Written as a learning exercise.

## Usage

### With Docker

```bash
  $ export TELEGRAM_BOT_TOKEN=your_token
  $ export TELEGRAM_CHAT_ID=your_chat_id
  $ docker build -t hn500 .
  $ docker run --name hn500 -e TELEGRAM_BOT_TOKEN -e TELEGRAM_CHAT_ID hn500
```

### With Docker Compose

```bash
  $ export TELEGRAM_BOT_TOKEN=your_token
  $ export TELEGRAM_CHAT_ID=your_chat_id
  $ docker-compose up -d
```

## Configuration

The following environment variables are used to configure the bot:

*   `TELEGRAM_BOT_TOKEN`: Your Telegram bot token. You can get one by talking to the [BotFather](https://t.me/botfather).
*   `TELEGRAM_CHAT_ID`: The ID of the chat where you want to send the stories. You can get this by talking to the `@userinfobot` bot.

## Project Architecture

The project is structured as follows:

*   `src/main.rs`: The entry point of the application.
*   `src/client.rs`: Handles fetching the top 500 stories from Hacker News.
*   `src/broadcast.rs`: Handles sending the stories to the Telegram chat.
*   `src/config.rs`: Handles loading the configuration from the environment.
*   `src/models.rs`: Contains the data models for the Hacker News stories.
*   `src/utils.rs`: Contains utility functions.
*   `src/conversions.rs`: Contains functions for converting between different data models.

## TODO

- [x] Add tests
- [x] Add CI
- [x] Add logging
- [x] Add Dockerfile
- [x] Add documentation
