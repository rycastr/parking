use crate::domain::error::ModelError;

pub enum AppError {
    UnprocessableEntity(&'static str),
    NotFound,
    Conflict,
    InternalServerError,
}

impl From<ModelError> for AppError {
    fn from(error: ModelError) -> Self {
        let reason = match error {
            ModelError::SessionAlreadyClosed => "SessionAlreadyClosed",
            ModelError::InvalidCheckoutDate => "InvalidCheckoutDate",
        };
        Self::UnprocessableEntity(reason)
    }
}
