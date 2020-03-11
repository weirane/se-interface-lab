use actix_http::ResponseBuilder;
use actix_web::error::ResponseError;
use actix_web::{http::StatusCode, HttpResponse};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum Errors {
    #[error("uesr exists")]
    UserExists,
    #[error("invalid login")]
    InvalidLogin,
    #[error("invalid token")]
    InvalidToken,
    #[error("invalid date")]
    InvalidDate,
    #[error("database error: {0}")]
    DBError(#[from] diesel::result::Error),
    #[error("database connection error")]
    DBConnError,
}

impl ResponseError for Errors {
    fn error_response(&self) -> HttpResponse {
        let res = json!({
            "success": false,
            "reason": format!("{}", self),
        });
        ResponseBuilder::new(self.status_code()).body(res.to_string())
    }

    fn status_code(&self) -> StatusCode {
        use Errors::*;
        match self {
            DBError(_) | DBConnError => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::OK,
        }
    }
}
