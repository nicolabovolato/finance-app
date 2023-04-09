use async_trait::async_trait;
use uuid::Uuid;

use crate::application::services::mail::MailService;
use crate::application::services::otp::OtpService;
use crate::application::services::tokens::TokenService;
use crate::application::services::users::UserService;
use crate::domain::entities::auth::Claims;
use crate::domain::entities::users::User;
use crate::domain::error::Result;

#[async_trait]
pub trait AuthUseCaseTrait: Send + Sync {
    async fn send_otp(&self, email: &str) -> Result<()>;
    async fn validate_token(&self, token: &str) -> Result<Claims>;
    async fn login(&self, email: &str, otp: &str) -> Result<String>;
    async fn signup(&self, email: &str, otp: &str) -> Result<()>;
}

pub struct AuthUseCase {
    mail_service: Box<dyn MailService>,
    otp_service: Box<dyn OtpService>,
    token_service: Box<dyn TokenService>,
    user_service: Box<dyn UserService>,
}

impl AuthUseCase {
    pub fn new(
        otp_service: Box<dyn OtpService>,
        mail_service: Box<dyn MailService>,
        token_service: Box<dyn TokenService>,
        user_service: Box<dyn UserService>,
    ) -> Self {
        Self {
            mail_service,
            otp_service,
            token_service,
            user_service,
        }
    }
}

#[async_trait]
impl AuthUseCaseTrait for AuthUseCase {
    async fn send_otp(&self, email: &str) -> Result<()> {
        let otp = self.otp_service.generate_otp_for(email).await?;
        self.mail_service
            .send_email(email, "OTP Token", &format!("Your OTP is <b>{otp}</b>"))
            .await?;
        Ok(())
    }

    async fn validate_token(&self, token: &str) -> Result<Claims> {
        let claims = self.token_service.validate(token).await?;
        Ok(claims)
    }

    async fn login(&self, email: &str, otp: &str) -> Result<String> {
        let user = self.user_service.find_by_email(email).await?;
        self.otp_service.validate(email, otp).await?;

        let access_token = self.token_service.generate(Claims { sub: user.id }).await?;
        Ok(access_token)
    }

    async fn signup(&self, email: &str, otp: &str) -> Result<()> {
        self.otp_service.validate(email, otp).await?;
        self.user_service
            .insert(User {
                id: Uuid::new_v4(),
                email: email.to_string(),
            })
            .await?;
        Ok(())
    }
}

#[cfg(test)]
use mockall::*;
#[cfg(test)]
mock! {
    pub AuthUseCase {}
    #[async_trait]
    impl AuthUseCaseTrait for AuthUseCase {
        async fn send_otp(&self, email: &str) -> Result<()>;
        async fn validate_token(&self, token: &str) -> Result<Claims>;
        async fn login(&self, email: &str, otp: &str) -> Result<String>;
        async fn signup(&self, email: &str, otp: &str) -> Result<()>;
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate;
    use tokio;

    use super::*;
    use crate::application::services::mail::MockMailService;
    use crate::application::services::otp::MockOtpService;
    use crate::application::services::tokens::MockTokenService;
    use crate::application::services::users::MockUserService;

    fn get_mock_use_case(
        mail_service: MockMailService,
        otp_service: MockOtpService,
        token_service: MockTokenService,
        user_service: MockUserService,
    ) -> AuthUseCase {
        AuthUseCase {
            mail_service: Box::new(mail_service),
            otp_service: Box::new(otp_service),
            token_service: Box::new(token_service),
            user_service: Box::new(user_service),
        }
    }

    #[tokio::test]
    async fn send_otp_successful() {
        let email = "somebody@somebody.com";
        let otp = "123456";

        let mut otp_service = MockOtpService::new();
        otp_service
            .expect_generate_otp_for()
            .with(predicate::eq(email))
            .return_once(|_| Ok(otp.to_string()));

        let mut mail_service = MockMailService::new();
        mail_service
            .expect_send_email()
            .with(
                predicate::eq(email),
                predicate::always(),
                predicate::function(move |x: &str| x.contains(otp)),
            )
            .return_once(|_, _, _| Ok(()));

        let use_case = get_mock_use_case(
            mail_service,
            otp_service,
            MockTokenService::new(),
            MockUserService::new(),
        );

        use_case.send_otp(email).await.unwrap();
    }

    #[tokio::test]
    async fn validate_token_successful() {
        let token = "token";
        let claims = Claims {
            sub: Uuid::new_v4(),
        };
        let claims2 = claims.clone();

        let mut token_service = MockTokenService::new();
        token_service
            .expect_validate()
            .with(predicate::eq(token))
            .return_once(|_| Ok(claims));

        let use_case = get_mock_use_case(
            MockMailService::new(),
            MockOtpService::new(),
            token_service,
            MockUserService::new(),
        );

        let result = use_case.validate_token(token).await.unwrap();
        assert_eq!(result, claims2);
    }

    #[tokio::test]
    async fn login_successful() {
        let email = "somebody@somebody.com";
        let otp = "123456";
        let token = "token";
        let user_id = Uuid::new_v4();

        let mut otp_service = MockOtpService::new();
        otp_service
            .expect_validate()
            .with(predicate::eq(email), predicate::eq(otp))
            .return_once(|_, _| Ok(()));

        let mut user_service = MockUserService::new();
        user_service
            .expect_find_by_email()
            .with(predicate::eq(email))
            .return_once(move |_| {
                Ok(User {
                    id: user_id,
                    email: email.to_string(),
                })
            });

        let mut token_service = MockTokenService::new();
        token_service
            .expect_generate()
            .with(predicate::eq(Claims { sub: user_id }))
            .return_once(|_| Ok(token.to_string()));

        let use_case = get_mock_use_case(
            MockMailService::new(),
            otp_service,
            token_service,
            user_service,
        );

        let result = use_case.login(email, otp).await.unwrap();
        assert_eq!(result, token);
    }

    #[tokio::test]
    async fn signup_successful() {
        let email = "somebody@somebody.com";
        let otp = "123456";
        let user_id = Uuid::new_v4();

        let mut otp_service = MockOtpService::new();
        otp_service
            .expect_validate()
            .with(predicate::eq(email), predicate::eq(otp))
            .return_once(|_, _| Ok(()));

        let mut user_service = MockUserService::new();
        user_service
            .expect_insert()
            .with(predicate::function(move |u: &User| u.email == email))
            .return_once(move |_| {
                Ok(User {
                    id: user_id,
                    email: email.to_string(),
                })
            });

        let use_case = get_mock_use_case(
            MockMailService::new(),
            otp_service,
            MockTokenService::new(),
            user_service,
        );

        use_case.signup(email, otp).await.unwrap();
    }
}
