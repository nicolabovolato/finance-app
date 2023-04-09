use async_trait::async_trait;
use chrono::{Duration, Utc};
use pasetors::{
    claims::{Claims as PasetoClaims, ClaimsValidationRules},
    keys::{AsymmetricKeyPair, AsymmetricPublicKey, AsymmetricSecretKey},
    public,
    token::UntrustedToken,
    version4::V4,
    Public,
};
use uuid::Uuid;

use crate::application::services::tokens::TokenService;
use crate::domain::entities::auth::Claims;
use crate::domain::error::{AuthErrorType, Error, Result};

mod error;

pub struct PasetoTokenService {
    keypair: AsymmetricKeyPair<V4>,
    expiration: Duration,
}

impl PasetoTokenService {
    pub fn new(public_key_bytes: &[u8], private_key_bytes: &[u8], expiration: Duration) -> Self {
        Self {
            keypair: AsymmetricKeyPair {
                public: AsymmetricPublicKey::from(public_key_bytes).unwrap(),
                secret: AsymmetricSecretKey::from(&[private_key_bytes, public_key_bytes].concat())
                    .unwrap(),
            },
            expiration,
        }
    }
}

#[async_trait]
impl TokenService for PasetoTokenService {
    async fn generate(&self, claims: Claims) -> Result<String> {
        let exp = Utc::now() + self.expiration;
        let mut paseto_claims = PasetoClaims::new()?;
        paseto_claims.expiration(&exp.to_rfc3339())?;
        paseto_claims.subject(&claims.sub.to_string())?;
        let token = public::sign(&self.keypair.secret, &paseto_claims, None, None)?;
        Ok(token)
    }

    async fn validate(&self, token: &str) -> Result<Claims> {
        let claims = ClaimsValidationRules::new();
        let token = UntrustedToken::<Public, V4>::try_from(token)?;
        let trusted_token = public::verify(&self.keypair.public, &token, &claims, None, None)?;

        let claims = &trusted_token
            .payload_claims()
            .ok_or(Error::Auth(AuthErrorType::InvalidToken))?;

        let sub = claims
            .get_claim("sub")
            .ok_or(Error::Auth(AuthErrorType::InvalidToken))?;

        let sub = serde_json::from_value::<String>(sub.clone())
            .map_err(|_| Error::Auth(AuthErrorType::InvalidToken))?;

        Ok(Claims {
            sub: Uuid::parse_str(&sub).map_err(|_| Error::Auth(AuthErrorType::InvalidToken))?,
        })
    }
}

#[cfg(test)]
mod integration_tests {

    use super::*;
    use ed25519::{
        pkcs8::{DecodePrivateKey, DecodePublicKey, PublicKeyBytes},
        KeypairBytes,
    };
    use once_cell::sync::Lazy;

    const PUBLIC_KEY: Lazy<Vec<u8>> = Lazy::new(|| {
        PublicKeyBytes::from_public_key_pem(
            "-----BEGIN PUBLIC KEY-----\n\
            MCowBQYDK2VwAyEAsL3ElqFG7ELhKdqz82cExUkmu+t0fy2yPd7rmAxhn/Y=\n\
            -----END PUBLIC KEY-----",
        )
        .unwrap()
        .to_bytes()
        .into()
    });
    const PRIVATE_KEY: Lazy<Vec<u8>> = Lazy::new(|| {
        KeypairBytes::from_pkcs8_pem(
            "-----BEGIN PRIVATE KEY-----\n\
            MC4CAQAwBQYDK2VwBCIEIKHLJM3ffad6X6Z9OflMjVo0kCxYbI7vlxKujreBderF\n\
            -----END PRIVATE KEY-----",
        )
        .unwrap()
        .secret_key
        .into()
    });

    const EXPIRATION: Lazy<chrono::Duration> = Lazy::new(|| chrono::Duration::minutes(1));

    #[tokio::test]
    async fn generate_success() {
        let service = PasetoTokenService::new(&PUBLIC_KEY, &PRIVATE_KEY, *EXPIRATION);
        service
            .generate(Claims {
                sub: Uuid::new_v4(),
            })
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn validate_success() {
        let service = PasetoTokenService::new(&PUBLIC_KEY, &PRIVATE_KEY, *EXPIRATION);
        let uuid = Uuid::new_v4();
        let claims = Claims { sub: uuid };
        let token = service.generate(claims).await.unwrap();
        let result = service.validate(&token).await.unwrap();
        assert_eq!(uuid, result.sub)
    }

    #[tokio::test]
    #[should_panic(expected = "Auth(InvalidToken)")]
    async fn token_expires() {
        let service =
            PasetoTokenService::new(&PUBLIC_KEY, &PRIVATE_KEY, chrono::Duration::microseconds(1));
        let token = service
            .generate(Claims {
                sub: Uuid::new_v4(),
            })
            .await
            .unwrap();
        service.validate(&token).await.unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "Auth(InvalidToken)")]
    async fn empty_token_error() {
        let service = PasetoTokenService::new(&PUBLIC_KEY, &PRIVATE_KEY, *EXPIRATION);
        service.validate("").await.unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "Auth(InvalidToken)")]
    async fn invalid_token_error() {
        let service = PasetoTokenService::new(&PUBLIC_KEY, &PRIVATE_KEY, *EXPIRATION);
        service.validate("notatoken").await.unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "Auth(InvalidToken)")]
    async fn forged_token_error() {
        let service = PasetoTokenService::new(&PUBLIC_KEY, &PRIVATE_KEY, *EXPIRATION);
        let token = "v4.public.eyJzdWIiOiJ1c2VyIiwiZXhwIjoiMjAyMy0wMy0yNVQxODo0NjoyOS42ODBaIn1DJzRYuirQ_fN7hHPRADWaQ7jNQQIaS1Co0rzp6jdGNH8wdcR6WPqlbjlN9lLeyROlDDXrpLwYJASoNSQx4WEH";
        service.validate(token).await.unwrap();
    }
}
