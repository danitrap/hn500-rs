# Hacker News 500 Telegram Bot

This is a Rust program that fetches the top 500 stories from Hacker News and
sends them to a Telegram chat.

It deduplicates the stories and only sends the new ones.

Written as a learning exercise.

## Usage

```bash
  $ export TELEGRAM_BOT_TOKEN=your_token
  $ export TELEGRAM_CHAT_ID=your_chat_id
  $ docker build -t hn500 .
  $ docker run --name hn500 -e TELEGRAM_BOT_TOKEN -e TELEGRAM_CHAT_ID hn500
```

## TODO

- [x] Add tests
- [x] Add CI
- [x] Add logging
- [x] Add Dockerfile
- [ ] Add documentation
