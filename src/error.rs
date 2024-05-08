use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    DatabaseError,
    InvalidToken,
    AuthHeaderNotFound,
    ServerError,
    NotFoundError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid Token"),
            Self::AuthHeaderNotFound => (StatusCode::BAD_REQUEST, "Authorization header not found"),
            Self::ServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "The server encountered an internal error while processing this request.",
            ),
            Self::DatabaseError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "The server encountered an internal error while processing this request.",
            ),
            Self::NotFoundError => (StatusCode::NOT_FOUND, "Invalid Request.Resource Not Found"),
        };
        (status, Json(json!({ "error": err_msg }))).into_response()
    }
}
