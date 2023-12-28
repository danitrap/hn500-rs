# Hacker News 500 Telegram Bot

This is a Rust program that fetches the top 500 stories from Hacker News and
sends them to a Telegram chat.

It deduplicates the stories and only sends the new ones.

Written as a learning exercise.

## Usage

```bash
  $ export BOT_TOKEN=your_token
  $ export CHAT_ID=your_chat_id
  $ cargo run
```

## TODO

- [x] Add tests
- [x] Add CI
- [x] Add logging
- [ ] Add Dockerfile
- [ ] Add documentation
