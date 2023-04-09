use async_trait::async_trait;

use super::KVStore;
use crate::domain::error::Result;

#[async_trait]
pub trait OtpService: KVStore<String> + Send + Sync {
    async fn generate_otp_for(&self, key: &str) -> Result<String>;
    async fn validate(&self, key: &str, otp: &str) -> Result<()>;
}

#[cfg(test)]
use chrono::Duration;
#[cfg(test)]
use mockall::*;
#[cfg(test)]
mock! {
    pub OtpService {}
    #[async_trait]
    impl KVStore<String> for OtpService {
        async fn get(&self, key: &str, delete: bool) -> Result<Option<String>>;
        async fn set(&self, key: &str, value: String, expiration: Option<Duration>) -> Result<String>;
    }
    #[async_trait]
    impl OtpService for OtpService {
        async fn generate_otp_for(&self, key: &str) -> Result<String>;
        async fn validate(&self, key: &str, otp: &str) -> Result<()>;
    }
}
