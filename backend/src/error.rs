use actix_web::{HttpResponse, ResponseError};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use derive_more::{Display};

#[derive(Debug, Display)]
pub enum ApiError {
    UserNotFound,
    UserUpdateFailure,
    UserCreateFailure,
    BadUserRequest
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::UserNotFound => StatusCode::NOT_FOUND,
            ApiError::UserUpdateFailure => StatusCode::FAILED_DEPENDENCY,
            ApiError::UserCreateFailure => StatusCode::FAILED_DEPENDENCY,
            ApiError::BadUserRequest => StatusCode::BAD_REQUEST
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}
