use async_trait::async_trait;
use axum::{
    extract::{rejection::JsonRejection, FromRef, FromRequest, FromRequestParts, Json},
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, Request},
    RequestPartsExt, TypedHeader,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::infrastructure::web::State;
use crate::{domain::entities::auth::Claims, domain::error::Error};

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, B, Rejection = JsonRejection>,
    B: Send + 'static,
{
    type Rejection = Error;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedJson(value))
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    State: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = State::from_ref(state);
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await?;
        let claims = state.auth.validate_token(bearer.token()).await?;

        Ok(claims)
    }
}
