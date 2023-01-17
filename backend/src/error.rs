use actix_web::{HttpResponse, ResponseError};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use derive_more::{Display};

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("Not found error")]
    UserNotFound,
    #[error("Bad request error")]
    UserUpdateFailure,
    #[error("Bad request error")]
    UserCreateFailure,
    #[error("Bad request error")]
    BadUserRequest,
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error)
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::UserNotFound => StatusCode::NOT_FOUND,
            ApiError::UserUpdateFailure => StatusCode::FAILED_DEPENDENCY,
            ApiError::UserCreateFailure => StatusCode::FAILED_DEPENDENCY,
            ApiError::BadUserRequest => StatusCode::BAD_REQUEST,
            ApiError::DatabaseError(..) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}
