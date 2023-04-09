use async_trait::async_trait;
use bb8_redis::{bb8::Pool, redis::AsyncCommands, RedisConnectionManager};
use chrono::Duration;
use rand::{self, Rng};

use crate::application::services::otp::OtpService;
use crate::application::services::KVStore;
use crate::domain::error::{AuthErrorType, Error, Result};

mod error;

pub struct RedisOtpService {
    pool: Pool<RedisConnectionManager>,
}

impl RedisOtpService {
    pub fn new(pool: Pool<RedisConnectionManager>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OtpService for RedisOtpService {
    async fn generate_otp_for(&self, key: &str) -> Result<String> {
        let otp = format!("{:06}", rand::thread_rng().gen_range(0..700_000));
        let otp = &self.set(key, otp, Some(Duration::minutes(10))).await?;
        Ok(otp.to_string())
    }

    async fn validate(&self, key: &str, otp: &str) -> Result<()> {
        let stored_otp = self.get(key, true).await?;

        match stored_otp {
            Some(x) if x == otp => Ok(()),
            _ => Err(Error::Auth(AuthErrorType::InvalidOtp)),
        }
    }
}

#[async_trait]
impl KVStore<String> for RedisOtpService {
    async fn get(&self, key: &str, delete: bool) -> Result<Option<String>> {
        let mut conn = self.pool.get().await?;
        let value = match delete {
            true => conn.get_del(key).await?,
            false => conn.get(key).await?,
        };
        Ok(value)
    }

    async fn set(&self, key: &str, value: String, expiration: Option<Duration>) -> Result<String> {
        let mut conn = self.pool.get().await?;
        match expiration {
            Some(expiration) => {
                conn.set_ex(key, &value, expiration.num_seconds() as usize)
                    .await?
            }
            None => conn.set(key, &value).await?,
        };
        Ok(value)
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    use once_cell::sync::Lazy;
    use tokio::time::sleep;
    use uuid::Uuid;

    const REDIS_URL: &str = "redis://localhost:6379/0";

    fn get_pool() -> Pool<RedisConnectionManager> {
        Pool::builder().build_unchecked(
            RedisConnectionManager::new(REDIS_URL).expect("Error connecting to cache"),
        )
    }

    #[tokio::test]
    async fn get_none() {
        let service = RedisOtpService::new(get_pool());
        let key = Uuid::new_v4().to_string();
        assert_eq!(service.get(&key, false).await.unwrap(), None)
    }

    #[tokio::test]
    async fn set() {
        let service = RedisOtpService::new(get_pool());
        let key = Uuid::new_v4().to_string();
        let value = "value".to_string();
        assert_eq!(service.set(&key, value.clone(), None).await.unwrap(), value)
    }

    #[tokio::test]
    async fn set_get() {
        let service = RedisOtpService::new(get_pool());
        let key = Uuid::new_v4().to_string();
        let value = "value".to_string();
        service.set(&key, value.clone(), None).await.unwrap();
        assert_eq!(service.get(&key, false).await.unwrap(), Some(value))
    }

    #[tokio::test]
    async fn set_get_delete() {
        let service = RedisOtpService::new(get_pool());
        let key = Uuid::new_v4().to_string();
        service.set(&key, "value".to_string(), None).await.unwrap();
        service.get(&key, true).await.unwrap();
        assert_eq!(service.get(&key, false).await.unwrap(), None)
    }

    #[tokio::test]
    async fn set_expiration_get() {
        let service = RedisOtpService::new(get_pool());
        let key = Uuid::new_v4().to_string();
        service
            .set(
                &key,
                "value".to_string(),
                Some(chrono::Duration::seconds(1)),
            )
            .await
            .unwrap();
        sleep(chrono::Duration::seconds(1).to_std().unwrap()).await;
        assert_eq!(service.get(&key, false).await.unwrap(), None)
    }

    #[tokio::test]
    async fn generate_otp_for() {
        let service = RedisOtpService::new(get_pool());
        let key = Uuid::new_v4().to_string();
        service
            .set(
                &key,
                "value".to_string(),
                Some(chrono::Duration::seconds(1)),
            )
            .await
            .unwrap();
        sleep(chrono::Duration::seconds(1).to_std().unwrap()).await;
        let otp = service.generate_otp_for(&key).await.unwrap();
        assert_eq!(service.get(&key, false).await.unwrap(), Some(otp))
    }

    #[tokio::test]
    async fn validate() {
        let service = RedisOtpService::new(get_pool());
        let key = Uuid::new_v4().to_string();
        let otp = service.generate_otp_for(&key).await.unwrap();
        service.validate(&key, &otp).await.unwrap();
        assert_eq!(service.get(&key, false).await.unwrap(), None)
    }

    #[tokio::test]
    #[should_panic(expected = "Auth(InvalidOtp)")]
    async fn validate_error() {
        let service = RedisOtpService::new(get_pool());
        let key = Uuid::new_v4().to_string();
        let otp = "123456";
        service.set(&key, "000000".to_string(), None).await.unwrap();
        service.validate(&key, otp).await.unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "Auth(InvalidOtp)")]
    async fn validate_error_missing() {
        let service = RedisOtpService::new(get_pool());
        let key = Uuid::new_v4().to_string();
        let otp = "123456";
        service.validate(&key, otp).await.unwrap();
    }
}
