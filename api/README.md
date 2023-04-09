# Rust Service

Monolithic service that exposes basic functionality for the app:

- Manages users and accounts through a Postgres database
- Manages authentication and authorization with Paseto v4 public tokens
- Manages passwordless login and signup via OTPs sent via SMTP and stored in Redis

Run `cargo` for available commands.
See [.env.example](.env.example) for environment variables and [config.rs](src/config.rs) for defaults.

## Develop/run locally

- Run `docker compose -f docker-compose.dev.yaml up -d`

## Manually migrate the database

- Install sqlx-cli `cargo install sqlx-cli`
- Run `sqlx migrate run`

Run `sqlx` for additional commands

## Build the service without an active Postgres connection

- Set `SQLX_OFFLINE=true`
- Run `cargo build`

## View test coverage

- Follow steps for [develop locally](#develop-locally)
- Install grcov `cargo install grcov`
- Run `RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test`
- Run `grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o ./target/debug/coverage/`
- View [coverage](./target/debug/coverage/index.html) in your browser
