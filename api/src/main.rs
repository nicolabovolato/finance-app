use dotenvy::dotenv;

mod application;
mod config;
mod domain;
mod infrastructure;
mod log;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = envy::from_env::<config::Config>().expect("Error reading configuration");
    let _guard = log::init(&config.log_level, config.log_pretty);

    infrastructure::run(config).await;
}
