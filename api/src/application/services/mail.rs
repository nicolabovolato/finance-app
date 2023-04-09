use async_trait::async_trait;

use crate::domain::error::Result;

#[async_trait]
pub trait MailService: Send + Sync {
    async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<()>;
}

#[cfg(test)]
use mockall::*;

#[cfg(test)]
mock! {
    pub MailService {}
    #[async_trait]
    impl MailService for MailService {
        async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<()>;
    }
}
