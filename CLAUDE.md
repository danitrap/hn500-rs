# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust application that fetches the top 500 Hacker News stories via RSS feed and sends new stories to a Telegram chat. The application runs continuously, checking for new stories every 10 minutes while maintaining a deduplication system to track previously seen items.

## Development Commands

### Common Development Tasks
- `make run` - Run the application with dotenv support
- `make watch` - Run with hot reload using cargo-watch
- `make build` - Build release version
- `make lint` - Run Rust code linting
- `make test` - Run tests with hot reload
- `make test-ci` - Run tests once (CI mode)
- `make gemini-test` - Run comprehensive tests including Docker build
- `make test-docker` - Test Docker build specifically
- `make clean` - Clean build artifacts

### Configuration
The application requires environment variables:
- `TELEGRAM_BOT_TOKEN` - Telegram bot token
- `TELEGRAM_CHAT_ID` - Target chat ID

For development, use the `dotenv` feature: `cargo run --features=dotenv`

## Architecture

### Core Components
- **main.rs**: Entry point with 10-minute interval loop and error handling
- **client.rs**: Fetches RSS feed from `https://hnrss.org/newest?points=500`
- **broadcast.rs**: Sends messages to Telegram API
- **config.rs**: Environment variable configuration with optional dotenv support
- **models.rs**: Core data structures and deduplication logic
- **conversions.rs**: RSS to internal model conversion trait
- **utils.rs**: HTML stripping utility

### Key Design Patterns
- **Deduplication System**: `HackerNews` struct maintains a `HashSet` for O(1) lookups and `VecDeque` for chronological history (max 100 items)
- **Error Handling**: Custom `ApplicationError` enum for different failure modes
- **Async Architecture**: Uses `tokio` runtime with `reqwest::Client` reuse for HTTP efficiency
- **Trait-based Conversions**: `ConvertibleToHnItem` trait for RSS item processing

### Data Flow
1. Fetch RSS feed bytes from Hacker News
2. Parse RSS using `rss` crate
3. Convert RSS items to `HnItem` via trait
4. Check for new items using deduplication system
5. Send new items to Telegram (skips first run)

## Testing

The codebase includes comprehensive unit tests in each module. Tests cover:
- HTML stripping functionality
- RSS to HnItem conversion
- Deduplication logic and history management
- Edge cases for missing RSS fields

Run `make gemini-test` for the full test suite including Docker build validation.

## Code Style

- Uses `#![deny(clippy::all)]` for strict linting
- Follows Rust 2021 edition conventions
- Comprehensive documentation with module-level comments
- Error types are descriptive and context-specific
