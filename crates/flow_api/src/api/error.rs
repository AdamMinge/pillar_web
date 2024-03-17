use crate::types::ErrorResponse;

use thiserror::Error as ThisError;

#[derive(ThisError, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("Not Initialized")]
    NotInitialized,

    #[error("Configuration Error")]
    ConfigurationError(String),

    #[error("Storage Error")]
    StorageError(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Not Found")]
    NotFound,

    #[error("Unprocessable Entity: {0:?}")]
    UnprocessableEntity(ErrorResponse),

    #[error("Internal Server Error")]
    InternalServerError,

    #[error("Deserialize Error")]
    DeserializeError,

    #[error("Http Request Error")]
    RequestError,
}
