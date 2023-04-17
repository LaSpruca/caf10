use actix::MailboxError;
use actix_web::{body::BoxBody, http::StatusCode, HttpResponseBuilder, ResponseError};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("There is no game with the given game code")]
    NoGameCode,
    #[error("Sorry but this name is already in use")]
    NameTaken,
    #[error("Internal server error")]
    InternalServerError,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> actix_web::HttpResponse<BoxBody> {
        let status = match self {
            ApiError::NoGameCode => StatusCode::NOT_FOUND,
            ApiError::NameTaken => StatusCode::CONFLICT,
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        HttpResponseBuilder::new(status).json(serde_json::json!({
            "error": self.to_string(),
        }))
    }
}

impl From<MailboxError> for ApiError {
    fn from(_: MailboxError) -> Self {
        ApiError::InternalServerError
    }
}
