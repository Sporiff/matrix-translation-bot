# matrix-translation-bot

A translation bot for Matrix written in Rust

![Example](examples/Example.webm)

## Translate Conversations Using Yandex

This bot allows you to call on a bot to provide
translations from one language to another using
the Yandex translate API. This bot will listen
for a given command with a given prefix and then
a destination language code (2 characters, e.g.
``fr`` for French). The API will automatically
attempt to guess the source language based on the
given string.

## Requirements

To use this bot you will need to obtain a key for
the [Yandex translation API](https://tech.yandex.com/translate/),
which will entitle you to 1,000,000 translatable
characters a day and 10,000,000 a month.

## Setup

To set up the bot:

Clone this repository

```
git clone https://gitlab.com/sporiff/matrix-translation-bot.git
```

Build the application using Rust (installing [Rustup](https://rustup.rs/)
is the easiest way of getting the tools you need)

```
cargo build --release
```

Copy the executable from the ``target/release`` folder to a
given location along with a ``botconfig.toml`` file with the
following information:

```
user = "bot_account_username"
password = "your_password_here"
homeserver_url = "https://matrix.org"
key = "yandex_api_key"
prefix = "%"
command = "translate"
```

In this example, the bot will listen for the command
``%translate <language code> <string>``

```
%translate fr hello!

bonjour!
```

You can also run this bot as a service following the example
unit file provided (systemd) or the service script (runit,
sysvinit)

## Acknowledgments

This bot is made using the following crates:

* [ytr](https://crates.io/crates/ytr)
* [matrix_bot_api](https://crates.io/crates/matrix_bot_api)