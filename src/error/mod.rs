use actix_web::{error, HttpResponse, http};
use diesel;

pub mod error_middleware;

#[derive(Fail, Debug)]
pub enum WebError {
    #[fail(display="Internal Server Error")]
    InternalError,
    #[fail(display="Database Error")]
    DatabaseError(diesel::result::Error),
}

impl error::ResponseError for WebError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            WebError::InternalError => HttpResponse::new(
                http::StatusCode::INTERNAL_SERVER_ERROR
            ),
            WebError::DatabaseError(_) => HttpResponse::new(
                http::StatusCode::INTERNAL_SERVER_ERROR
            )
        }
    }
}