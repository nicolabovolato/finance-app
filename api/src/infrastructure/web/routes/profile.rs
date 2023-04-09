use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::infrastructure::web::middleware::ValidatedJson;
use crate::infrastructure::web::State as AppState;
use crate::{
    domain::entities::{
        accounts::{Account, CategoryType, CurrencyType},
        auth::Claims,
    },
    domain::error::Error,
};

#[derive(Serialize)]
struct ProfileResponse {
    accounts: Vec<Account>,
}

#[derive(Deserialize, Validate)]
struct AccountBody {
    #[validate(length(min = 3, max = 64))]
    name: String,
    currency: CurrencyType,
}

#[derive(Deserialize, Validate)]
struct MovmentBody {
    #[validate(length(min = 3, max = 64))]
    title: String,
    category: CategoryType,
    amount: Decimal,
}

async fn get_profile(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<impl IntoResponse, Error> {
    let accounts = state.profile.get_accounts(claims.sub).await?;
    Ok((StatusCode::OK, Json(ProfileResponse { accounts })))
}

async fn get_account(
    State(state): State<AppState>,
    Path(account_id): Path<Uuid>,
    claims: Claims,
) -> Result<impl IntoResponse, Error> {
    let account = state.profile.get_account(claims.sub, account_id).await?;
    Ok((StatusCode::OK, Json(account)))
}

async fn post_account(
    State(state): State<AppState>,
    claims: Claims,
    ValidatedJson(payload): ValidatedJson<AccountBody>,
) -> Result<impl IntoResponse, Error> {
    let account = state
        .profile
        .create_account(claims.sub, &payload.name, payload.currency)
        .await?;

    Ok((StatusCode::CREATED, Json(account)))
}

async fn get_movements(
    State(state): State<AppState>,
    Path(account_id): Path<Uuid>,
    claims: Claims,
) -> Result<impl IntoResponse, Error> {
    let movements = state.profile.get_movements(claims.sub, account_id).await?;

    Ok((StatusCode::OK, Json(movements)))
}

async fn post_movement(
    State(state): State<AppState>,
    Path(account_id): Path<Uuid>,
    claims: Claims,
    ValidatedJson(payload): ValidatedJson<MovmentBody>,
) -> Result<impl IntoResponse, Error> {
    let movement = state
        .profile
        .create_movement(
            claims.sub,
            account_id,
            &payload.title,
            payload.category,
            payload.amount,
        )
        .await?;

    Ok((StatusCode::CREATED, Json(movement)))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_profile))
        .route("/accounts", post(post_account))
        .route("/accounts/:account_id", get(get_account))
        .route("/accounts/:account_id/movements", get(get_movements))
        .route("/accounts/:account_id/movements", post(post_movement))
}

#[cfg(test)]
mod tests {
    use axum::response::IntoResponse;
    use mockall::predicate;
    use rust_decimal::Decimal;
    use serde_json::{json, Value};

    use super::*;
    use crate::{
        application::use_cases::auth::MockAuthUseCase,
        application::use_cases::profile::MockProfileUseCase,
        domain::entities::accounts::{Account, Movement},
        domain::entities::auth::Claims,
        infrastructure::web::get_mock_state,
    };

    #[tokio::test]
    async fn get_profile_successful() {
        let user_id = uuid::Uuid::new_v4();
        let accounts = vec![Account {
            balance: Decimal::from(0),
            currency: CurrencyType::Usd,
            id: uuid::Uuid::new_v4(),
            user_id,
            name: "name".to_string(),
        }];
        let accounts2 = accounts.clone();

        let mut profile = MockProfileUseCase::new();
        profile
            .expect_get_accounts()
            .with(predicate::eq(user_id))
            .return_once(|_| Ok(accounts));

        let state = get_mock_state(MockAuthUseCase::new(), profile);

        let response = super::get_profile(axum::extract::State(state), Claims { sub: user_id })
            .await
            .unwrap()
            .into_response();

        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert_eq!(
            serde_json::from_slice::<Value>(
                &hyper::body::to_bytes(response.into_body()).await.unwrap()
            )
            .unwrap(),
            json!({ "accounts": accounts2 })
        );
    }

    #[tokio::test]
    async fn post_account_successful() {
        let user_id = uuid::Uuid::new_v4();
        let name = "name".to_string();
        let currency = CurrencyType::Usd;
        let account = Account {
            balance: Decimal::from(0),
            currency: currency.clone(),
            id: uuid::Uuid::new_v4(),
            user_id,
            name: name.clone(),
        };
        let account2 = account.clone();

        let mut profile = MockProfileUseCase::new();
        profile
            .expect_create_account()
            .with(
                predicate::eq(user_id),
                predicate::eq(name.clone()),
                predicate::eq(currency.clone()),
            )
            .return_once(|_, _, _| Ok(account));

        let state = get_mock_state(MockAuthUseCase::new(), profile);

        let response = super::post_account(
            axum::extract::State(state),
            Claims { sub: user_id },
            ValidatedJson(AccountBody { currency, name }),
        )
        .await
        .unwrap()
        .into_response();

        assert_eq!(response.status(), axum::http::StatusCode::CREATED);

        let body = serde_json::from_slice::<Account>(
            &hyper::body::to_bytes(response.into_body()).await.unwrap(),
        )
        .unwrap();

        assert_eq!(body, account2);
    }

    #[tokio::test]
    async fn get_account_successful() {
        let user_id = uuid::Uuid::new_v4();
        let account_id = uuid::Uuid::new_v4();
        let account = Account {
            balance: Decimal::from(0),
            currency: CurrencyType::Usd,
            id: account_id,
            user_id,
            name: "name".to_string(),
        };
        let account2 = account.clone();

        let mut profile = MockProfileUseCase::new();
        profile
            .expect_get_account()
            .with(predicate::eq(user_id), predicate::eq(account_id))
            .return_once(|_, _| Ok(account));

        let state = get_mock_state(MockAuthUseCase::new(), profile);

        let response = super::get_account(
            axum::extract::State(state),
            axum::extract::Path(account_id),
            Claims { sub: user_id },
        )
        .await
        .unwrap()
        .into_response();

        assert_eq!(response.status(), axum::http::StatusCode::OK);

        let body = serde_json::from_slice::<Account>(
            &hyper::body::to_bytes(response.into_body()).await.unwrap(),
        )
        .unwrap();

        assert_eq!(body, account2);
    }

    #[tokio::test]
    async fn post_movement_successful() {
        let user_id = uuid::Uuid::new_v4();
        let account_id = uuid::Uuid::new_v4();
        let title = "title".to_string();
        let category = CategoryType::Generic;
        let amount = Decimal::from(0);
        let movement_id = uuid::Uuid::new_v4();
        let movement = Movement {
            account_id,
            id: movement_id,
            amount,
            category: category.clone(),
            timestamp: chrono::Utc::now(),
            title: title.clone(),
        };
        let movement2 = movement.clone();

        let mut profile = MockProfileUseCase::new();
        profile
            .expect_create_movement()
            .with(
                predicate::eq(user_id),
                predicate::eq(account_id),
                predicate::eq(title.clone()),
                predicate::eq(category.clone()),
                predicate::eq(amount),
            )
            .return_once(|_, _, _, _, _| Ok(movement));

        let state = get_mock_state(MockAuthUseCase::new(), profile);

        let response = super::post_movement(
            axum::extract::State(state),
            axum::extract::Path(account_id),
            Claims { sub: user_id },
            ValidatedJson(MovmentBody {
                amount,
                category,
                title,
            }),
        )
        .await
        .unwrap()
        .into_response();

        assert_eq!(response.status(), axum::http::StatusCode::CREATED);

        let body = serde_json::from_slice::<Movement>(
            &hyper::body::to_bytes(response.into_body()).await.unwrap(),
        )
        .unwrap();

        assert_eq!(body, movement2);
    }

    #[tokio::test]
    async fn get_movements_successful() {
        let user_id = uuid::Uuid::new_v4();
        let account_id = uuid::Uuid::new_v4();
        let movements = vec![Movement {
            account_id,
            id: uuid::Uuid::new_v4(),
            amount: Decimal::from(0),
            category: CategoryType::Generic,
            timestamp: chrono::Utc::now(),
            title: "title".to_string(),
        }];
        let movements2 = movements.clone();

        let mut profile = MockProfileUseCase::new();
        profile
            .expect_get_movements()
            .with(predicate::eq(user_id), predicate::eq(account_id))
            .return_once(|_, _| Ok(movements));

        let state = get_mock_state(MockAuthUseCase::new(), profile);

        let response = super::get_movements(
            axum::extract::State(state),
            axum::extract::Path(account_id),
            Claims { sub: user_id },
        )
        .await
        .unwrap()
        .into_response();

        assert_eq!(response.status(), axum::http::StatusCode::OK);

        let body = serde_json::from_slice::<Vec<Movement>>(
            &hyper::body::to_bytes(response.into_body()).await.unwrap(),
        )
        .unwrap();

        assert_eq!(body, movements2);
    }
}
