use std::sync::Arc;
use tokio::signal;

use crate::application::use_cases::auth::AuthUseCase;
use crate::application::use_cases::profile::ProfileUseCase;
use crate::config::Config;

mod paseto;
mod pg;
mod redis;
mod smtp;
mod web;

pub async fn run(config: Config) {
    let shutdown_signal = shutdown_signal();

    let pg_pool = config.get_pg_pool();
    let redis_pool = config.get_redis_pool();
    let smtp_client = config.get_smtp_client();

    let (paseto_public_key, paseto_private_key, paseto_expiration) = config.get_paseto_config();

    sqlx::migrate!("./migrations")
        .run(&pg_pool)
        .await
        .expect("Failed to execute migrations");

    let token_service = Box::new(paseto::PasetoTokenService::new(
        &paseto_public_key,
        &paseto_private_key,
        paseto_expiration,
    ));
    let otp_service = Box::new(redis::RedisOtpService::new(redis_pool));
    let mail_service = Box::new(smtp::SmtpMailService::new(smtp_client));
    let user_service = Box::new(pg::users::PgUserService::new(pg_pool.clone()));
    let account_service = Box::new(pg::accounts::PgAccountService::new(pg_pool));

    let auth = AuthUseCase::new(otp_service, mail_service, token_service, user_service);
    let profile = ProfileUseCase::new(account_service);

    web::run(
        config,
        web::State::new(Arc::new(auth), Arc::new(profile)),
        shutdown_signal,
    )
    .await;
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Shutting down...");
}
