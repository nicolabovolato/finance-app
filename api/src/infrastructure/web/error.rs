use std::collections::HashMap;

use axum::{
    extract::rejection::{JsonRejection, TypedHeaderRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::domain::error::{AuthErrorType, Error, RepositoryErrorType};

impl From<TypedHeaderRejection> for Error {
    fn from(e: TypedHeaderRejection) -> Self {
        match e.name().to_string().to_lowercase().as_str() {
            "authorization" => Error::Auth(AuthErrorType::Missing),
            _ => Error::External(e.into()),
        }
    }
}

impl From<validator::ValidationErrors> for Error {
    fn from(e: validator::ValidationErrors) -> Self {
        Error::Validation(e.into())
    }
}

impl From<JsonRejection> for Error {
    fn from(e: JsonRejection) -> Self {
        Error::Validation(e.into())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        self.log();

        let msg = self.to_string();
        let payload = match self {
            Error::External(_) => {
                json!({
                    "error:": "Internal server error".to_string()
                })
            }
            Error::Validation(ref err) => match err.downcast_ref::<validator::ValidationErrors>() {
                Some(err) => {
                    let errors: HashMap<&str, String> = err
                        .field_errors()
                        .into_iter()
                        .map(|(field, err)| {
                            (
                                field,
                                err.iter()
                                    .map(|err| err.to_string().replace('\n', ","))
                                    .collect(),
                            )
                        })
                        .collect();
                    json!({ "error:": msg, "validation": errors })
                }
                _ => json!({ "error:": msg }),
            },
            _ => {
                json!({ "error:": msg })
            }
        };

        let status_code = match self {
            Error::Repository(RepositoryErrorType::NotFound) => StatusCode::NOT_FOUND,
            Error::Repository(RepositoryErrorType::Conflict) => StatusCode::CONFLICT,
            Error::Auth(_) => StatusCode::UNAUTHORIZED,
            Error::Validation(err) if matches!(err.downcast_ref::<JsonRejection>(), Some(_)) => {
                StatusCode::BAD_REQUEST
            }
            Error::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Error::External(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status_code, Json(payload)).into_response()
    }
}
