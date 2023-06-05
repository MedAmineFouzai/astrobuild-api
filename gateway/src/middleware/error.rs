
use async_graphql::Error;
use async_graphql::ErrorExtensions;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserCustomResponseError {
    #[error("Could not find resource")]
    NotFound,

    #[error("Server Error")]
    ServerError,

    #[error("Not Allowed")]
    NotAllowed,

    #[error("Conflict")]
    Conflict,
}

#[derive(Debug, Error)]
pub enum CategoryCustomResponseError {
    #[error("Could not find resource")]
    NotFound,

    #[error("Server Error")]
    ServerError,

    #[error("Not Allowed")]
    NotAllowed,
}

#[derive(Debug, Error)]
pub enum FeatureCustomResponseError {
    #[error("Could not find resource")]
    NotFound,

    #[error("Server Error")]
    ServerError,

    #[error("Not Allowed")]
    NotAllowed,
}
#[derive(Debug, Error)]
pub enum TemplateCustomResponseError {
    #[error("Could not find resource")]
    NotFound,

    #[error("Server Error")]
    ServerError,

    #[error("Not Allowed")]
    NotAllowed,
}

#[derive(Debug, Error)]
pub enum ProtoTypeCustomResponseError {
    #[error("Could not find resource")]
    NotFound,

    #[error("Server Error")]
    ServerError,

    #[error("Not Allowed")]
    NotAllowed,
}


#[derive(Debug, Error)]
pub enum ProjectCustomResponseError {
    #[error("Could not find resource")]
    NotFound,

    #[error("Server Error")]
    ServerError,

    #[error("Not Allowed")]
    NotAllowed,
}


impl ErrorExtensions for UserCustomResponseError {
    fn extend(&self) -> Error {
        Error::new(format!("{}", self)).extend_with(|_err, e| match self {
            UserCustomResponseError::NotFound => e.set("code", "NOT_FOUND"),
            UserCustomResponseError::Conflict => e.set("code", "Conflict"),
            UserCustomResponseError::ServerError => e.set("code", "Internal_Server_Error"),
            UserCustomResponseError::NotAllowed => e.set("code", "Not_Allowed"),
        })
    }
}

impl ErrorExtensions for CategoryCustomResponseError {
    fn extend(&self) -> Error {
        Error::new(format!("{}", self)).extend_with(|_err, e| match self {
            CategoryCustomResponseError::NotFound => e.set("code", "NOT_FOUND"),
            CategoryCustomResponseError::ServerError => e.set("code", "Internal_Server_Error"),
            CategoryCustomResponseError::NotAllowed => e.set("code", "Not_Allowed"),
        })
    }
}
