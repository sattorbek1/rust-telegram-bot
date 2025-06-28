# Rust Telegram Bot Example

A simple Telegram bot written in Rust using `teloxide`.

## 🚀 Getting Started

Follow these steps to run the bot locally:

### 1. Clone the repository

```bash
git clone https://github.com/sattorbek1/rust-telegram-bot.git rust-telegram-bot
```

### 2. Navigate into the project directory

```bash
cd rust-telegram-bot
```

### 3. Set your bot token in `src/main.rs`

Open `src/main.rs` and replace the placeholder with your actual Telegram Bot Token:

```rust
let bot = Bot::new("your_actual_telegram_bot_token_here");
```

> ⚠️ **Never share your token publicly.**

### 4. Run the bot

```bash
cargo run
```

## 🛠 Requirements

* [Rust](https://www.rust-lang.org/tools/install)
* A valid [Telegram Bot Token](https://t.me/BotFather)

## 📂 Project Structure

```
src/
├── handlers/
│   └── start.rs
├── main.rs
```

## ✅ Features

* Command handling
* Easily extensible
* Async & efficient using `tokio` and `teloxide`