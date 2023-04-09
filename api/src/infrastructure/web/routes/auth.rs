use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::error::Error;
use crate::infrastructure::web::middleware::ValidatedJson;
use crate::infrastructure::web::State as AppState;

#[derive(Deserialize, Validate)]
struct OtpBody {
    #[validate(email)]
    email: String,
}

#[derive(Deserialize, Validate)]
struct LoginBody {
    #[validate(email)]
    email: String,
    #[validate(length(equal = 6))]
    otp: String,
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    access_token: String,
}

async fn otp(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<OtpBody>,
) -> Result<impl IntoResponse, Error> {
    state.auth.send_otp(&payload.email).await?;
    Ok(StatusCode::CREATED)
}

async fn login(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<LoginBody>,
) -> Result<impl IntoResponse, Error> {
    let access_token = state.auth.login(&payload.email, &payload.otp).await?;

    Ok((StatusCode::CREATED, Json(LoginResponse { access_token })))
}

async fn signup(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<LoginBody>,
) -> Result<impl IntoResponse, Error> {
    state.auth.signup(&payload.email, &payload.otp).await?;
    Ok(StatusCode::CREATED)
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/otp", post(otp))
        .route("/signup", post(signup))
        .route("/login", post(login))
}

#[cfg(test)]
mod tests {
    use axum::response::IntoResponse;
    use mockall::predicate;
    use serde_json::{json, Value};

    use super::*;
    use crate::{
        application::use_cases::auth::MockAuthUseCase,
        application::use_cases::profile::MockProfileUseCase, infrastructure::web::get_mock_state,
    };

    #[tokio::test]
    async fn otp_successful() {
        let email = "somebody@somebody.com";

        let mut auth = MockAuthUseCase::new();
        auth.expect_send_otp()
            .with(predicate::eq(email))
            .return_once(|_| Ok(()));

        let state = get_mock_state(auth, MockProfileUseCase::new());

        assert_eq!(
            super::otp(
                axum::extract::State(state),
                ValidatedJson(super::OtpBody {
                    email: email.to_string(),
                }),
            )
            .await
            .into_response()
            .status(),
            axum::http::StatusCode::CREATED
        );
    }

    #[tokio::test]
    async fn login_successful() {
        let email = "somebody@somebody.com";
        let otp = "123456";
        let token = "token";

        let mut auth = MockAuthUseCase::new();
        auth.expect_login()
            .with(predicate::eq(email), predicate::eq(otp))
            .return_once(|_, _| Ok(token.to_string()));

        let state = get_mock_state(auth, MockProfileUseCase::new());

        let response = super::login(
            axum::extract::State(state),
            ValidatedJson(super::LoginBody {
                email: email.to_string(),
                otp: otp.to_string(),
            }),
        )
        .await
        .into_response();

        assert_eq!(response.status(), axum::http::StatusCode::CREATED);

        assert_eq!(
            serde_json::from_slice::<Value>(
                &hyper::body::to_bytes(response.into_body()).await.unwrap()
            )
            .unwrap(),
            json! ({
                "access_token": token.to_string()
            })
        );
    }

    #[tokio::test]
    async fn signup_successful() {
        let email = "somebody@somebody.com";
        let otp = "123456";

        let mut auth = MockAuthUseCase::new();
        auth.expect_signup()
            .with(predicate::eq(email), predicate::eq(otp))
            .return_once(|_, _| Ok(()));

        let state = get_mock_state(auth, MockProfileUseCase::new());

        let response = super::signup(
            axum::extract::State(state),
            ValidatedJson(super::LoginBody {
                email: email.to_string(),
                otp: otp.to_string(),
            }),
        )
        .await
        .into_response();

        assert_eq!(response.status(), axum::http::StatusCode::CREATED);
    }
}
