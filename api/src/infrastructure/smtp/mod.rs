use anyhow::anyhow;
use async_trait::async_trait;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

use crate::application::services::mail::MailService;
use crate::domain::error::{Error, Result};

mod error;

pub struct SmtpMailService {
    client: AsyncSmtpTransport<Tokio1Executor>,
}

impl SmtpMailService {
    pub fn new(client: AsyncSmtpTransport<Tokio1Executor>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl MailService for SmtpMailService {
    async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<()> {
        let mail = Message::builder()
            .from("noreply <noreply@personalfinanceapp.com>".parse()?)
            .to(to.parse()?)
            .subject(subject)
            .body(body.to_string())?;

        match self.client.send(mail).await?.is_positive() {
            true => Ok(()),
            false => Err(Error::External(anyhow!("Error sending email"))),
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use once_cell::sync::Lazy;

    const SMTP_HOST: &str = "localhost";
    const SMTP_PORT: u16 = 1025;

    static CLIENT: Lazy<AsyncSmtpTransport<Tokio1Executor>> = Lazy::new(|| {
        AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(SMTP_HOST)
            .port(SMTP_PORT)
            .build()
    });

    #[tokio::test]
    async fn send_email_success() {
        let service = SmtpMailService::new(CLIENT.to_owned());
        service.send_email("test@test.com", "", "").await.unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "Missing domain or user")]
    async fn send_email_error_missing_address() {
        let service = SmtpMailService::new(CLIENT.to_owned());
        service.send_email("", "", "").await.unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "Invalid email user")]
    async fn send_email_error_invalid_address() {
        let service = SmtpMailService::new(CLIENT.to_owned());
        service.send_email("whatever 123@", "", "").await.unwrap();
    }
}
