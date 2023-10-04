use std::fmt::Debug;

use diesel_async::pooled_connection::{
    deadpool::{BuildError, PoolError as DeadPoolPoolError},
    PoolError,
};
use serde::{Serialize, Serializer};
use thiserror::Error;

#[derive(Debug, Serialize, Error)]
pub enum ToDoError {
    #[error("User account already exists")]
    UserAccountAlreadyExists,
    #[error("Wrong credentials")]
    WrongCredentials,
    #[error("ToDo with such ID already exists")]
    ToDoWithIdAlreadyExists,
    #[error("Internal server error")]
    Internal(#[from] InternalError),
}

#[derive(Debug, Error)]
pub enum InternalError {
    #[error("Internal server error")]
    Build(#[from] BuildError),
    #[error("Internal server error")]
    Io(#[from] std::io::Error),
    #[error("Internal server error")]
    Pool(#[from] PoolError),
    #[error("Internal server error")]
    DeadPoolPool(#[from] DeadPoolPoolError),
    #[error("Internal server error")]
    DieselResult(#[from] diesel::result::Error),
}

impl Serialize for InternalError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}
