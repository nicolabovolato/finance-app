use async_trait::async_trait;

use crate::domain::entities::auth::Claims;
use crate::domain::error::Result;

#[async_trait]
pub trait TokenService: Send + Sync {
    async fn generate(&self, claims: Claims) -> Result<String>;
    async fn validate(&self, token: &str) -> Result<Claims>;
}

#[cfg(test)]
use mockall::*;

#[cfg(test)]
mock! {
    pub TokenService {}
    #[async_trait]
    impl TokenService for TokenService {
        async fn generate(&self, claims: Claims) -> Result<String>;
        async fn validate(&self, token: &str) -> Result<Claims>;
    }
}
