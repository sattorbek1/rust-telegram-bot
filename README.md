# Rust Telegram Bot Misoli

Rust dasturlash tilida `teloxide` kutubxonasi yordamida yozilgan oddiy Telegram bot.

## 🚀 Boshlash

Botni lokal kompyuteringizda ishga tushurish uchun quyidagi bosqichlarni bajaring:

### 1. Repositoriyani klonlash

```bash
git clone https://github.com/sattorbek1/rust-telegram-bot.git rust-telegram-bot
```

### 2. Loyihaning papkasiga o'ting

```bash
cd rust-telegram-bot
```

### 3. Bot tokenini `src/main.rs` fayliga kiriting

`src/main.rs` faylini oching va token o'rniga o'zingizning Telegram Bot Tokeningizni yozing:

```rust
let bot = Bot::new("your_actual_telegram_bot_token_here");
```

> ⚠️ **Tokeningizni hech qachon ommaga oshkor qilmang.**

### 4. Botni ishga tushuring

```bash
cargo run
```

## 🛠 Talablar

* [Rust](https://www.rust-lang.org/tools/install)
* Amalga oshirilgan [Telegram Bot Token](https://t.me/BotFather)

## 📂 Loyihaning tuzilishi

```
src/
├── handlers/
│   ├── mod.rs
│   ├── help.rs
│   ├── text.rs
│   └── start.rs
├── main.rs
```

## ✅ Imkoniyatlar

* Buyruqlarni qayta ishlash
* Oson kengaytiriladi
* `tokio` va `teloxide` yordamida asinxron va samarali ishlaydi