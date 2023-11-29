use axum::response::IntoResponse;
use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub), context(suffix(false)))]
pub enum EntityError {
    #[snafu(display("Entity not found"), context(false))]
    EntityNotFound { source: crate::model::DbError },
}

pub type EntityResult<T> = Result<T, EntityError>;

impl IntoResponse for EntityError {
    fn into_response(self) -> axum::response::Response {
        match self {
            EntityError::EntityNotFound { .. } => {
                (axum::http::StatusCode::NOT_FOUND, "Entity not found").into_response()
            }
        }
    }
}
