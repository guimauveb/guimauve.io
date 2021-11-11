/*
 * Map diesel errors to actix http responses.
 * Here we're only returning 404 when diesel returns NotFound. Anything else is considered a 500 (InternalServerError).
 * Since we're using actix_web::web::block to make database queries, we receive a diesel Error wrapped in a BlockingError<_>.
 */
use {
    actix_web::{error::BlockingError, HttpResponse, ResponseError},
    derive_more::{Display, From},
    diesel::result::Error as DieselError,
};

#[derive(Display, From, Debug)]
pub struct DatabaseError(pub BlockingError<DieselError>);

impl std::error::Error for DatabaseError {}

impl ResponseError for DatabaseError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            DatabaseError(BlockingError::Error(DieselError::NotFound)) => {
                HttpResponse::NotFound().finish()
            }
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}
