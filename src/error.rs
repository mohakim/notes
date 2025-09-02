use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize)]
struct ErrorBody {
    error: String,
    message: String,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("not found")]
    NotFound,
    #[error("database error")]
    Db(#[from] sqlx::Error),
    #[error("internal error")]
    Internal(anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, err, msg) = match self {
            AppError::NotFound | AppError::Db(sqlx::Error::RowNotFound) => {
                (StatusCode::NOT_FOUND, "not_found", "resource not found")
            }
            AppError::Db(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "db_error",
                "database error",
            ),
            AppError::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal",
                "internal server error",
            ),
        };

        let body = Json(ErrorBody {
            error: err.to_string(),
            message: msg.to_string(),
        });
        (status, body).into_response()
    }
}

impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        AppError::Internal(value)
    }
}

pub type AppResult<T> = Result<T, AppError>;
