use axum::http::{header, HeaderValue, Method};
use bb8_redis::{bb8::Pool, RedisConnectionManager};
use ed25519::pkcs8::{self, DecodePrivateKey, DecodePublicKey};
use lettre::{transport::smtp::authentication::Credentials, AsyncSmtpTransport, Tokio1Executor};
use serde::Deserialize;
use sqlx::PgPool;
use tower_http::cors::CorsLayer;

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_cors_origins")]
    pub cors_origins: Vec<String>,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default = "default_log_pretty")]
    pub log_pretty: bool,
    pub database_url: String,
    pub cache_url: String,
    #[serde(default = "default_paseto_expiration_minutes")]
    pub paseto_expiration_minutes: u32,
    pub paseto_public_key: String,
    pub paseto_private_key: String,
    pub smtp_host: String,
    #[serde(default = "default_smtp_port")]
    pub smtp_port: u16,
    pub smtp_username: Option<String>,
    pub smtp_password: Option<String>,
    #[serde(default = "default_smtp_secure")]
    pub smtp_secure: bool,
}

impl Config {
    pub fn get_pg_pool(&self) -> PgPool {
        PgPool::connect_lazy(&self.database_url).expect("Error connecting to db")
    }

    pub fn get_redis_pool(&self) -> Pool<RedisConnectionManager> {
        Pool::builder().build_unchecked(
            RedisConnectionManager::new(&*self.cache_url).expect("Error connecting to cache"),
        )
    }

    pub fn get_smtp_client(&self) -> AsyncSmtpTransport<Tokio1Executor> {
        let builder = match self.smtp_secure {
            true => AsyncSmtpTransport::<Tokio1Executor>::relay(&self.smtp_host).unwrap(),
            false => AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&self.smtp_host),
        };

        let builder = match (self.smtp_username.as_deref(), self.smtp_password.as_deref()) {
            (Some(username), Some(password)) => {
                builder.credentials(Credentials::new(username.to_string(), password.to_string()))
            }
            _ => builder,
        };

        builder.port(self.smtp_port).build()
    }

    pub fn get_paseto_config(&self) -> (Vec<u8>, Vec<u8>, chrono::Duration) {
        let public_key = pkcs8::PublicKeyBytes::from_public_key_pem(&self.paseto_public_key)
            .expect("Invalid EC25519 public key")
            .to_bytes();
        let private_key = pkcs8::KeypairBytes::from_pkcs8_pem(&self.paseto_private_key)
            .expect("Invalid EC25519 private key")
            .secret_key;

        let expiration = chrono::Duration::minutes(self.paseto_expiration_minutes.into());

        (public_key.into(), private_key.into(), expiration)
    }

    pub fn get_cors_layer(&self) -> CorsLayer {
        let cors = CorsLayer::new()
            .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]);

        match self.cors_origins.contains(&"*".to_string()) {
            true => cors.allow_origin(tower_http::cors::Any),
            false => cors.allow_origin(
                self.cors_origins
                    .iter()
                    .map(|origin| origin.parse::<HeaderValue>().expect("Invalid CORS Origin"))
                    .collect::<Vec<_>>(),
            ),
        }
    }
}

fn default_port() -> u16 {
    8080
}
fn default_cors_origins() -> Vec<String> {
    vec![]
}
fn default_log_level() -> String {
    "INFO".to_string()
}
fn default_log_pretty() -> bool {
    false
}
fn default_paseto_expiration_minutes() -> u32 {
    60
}
fn default_smtp_port() -> u16 {
    25
}
fn default_smtp_secure() -> bool {
    true
}
