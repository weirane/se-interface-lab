use actix_http::ResponseBuilder;
use actix_web::error::ResponseError;
use actix_web::{http::StatusCode, HttpResponse};
use log::{error, info};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Errors {
    #[error("user exists")]
    UserExists(String),
    #[error("invalid login")]
    InvalidLogin(String, String),
    #[error("invalid token")]
    InvalidToken(uuid::Uuid),
    #[error("invalid date")]
    InvalidDate(String),
    #[error("database error")]
    DBError(#[from] diesel::result::Error),
    #[error("database connection error")]
    DBConnError(#[from] r2d2::Error),
}

impl ResponseError for Errors {
    fn error_response(&self) -> HttpResponse {
        let code = self.status_code();
        if let StatusCode::INTERNAL_SERVER_ERROR = code {
            error!("{:?}", self);
        } else {
            info!("error: {:?}", self);
        }
        let res = json!({
            "success": false,
            "reason": format!("{}", self),
        });
        ResponseBuilder::new(code).json(res)
    }

    fn status_code(&self) -> StatusCode {
        use Errors::*;
        match self {
            DBError(_) | DBConnError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::OK,
        }
    }
}
