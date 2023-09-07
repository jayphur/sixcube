use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageErr{
    #[error("This error is just a test")]
    EmptyError,
    #[error("No type id is found for id listed in package")]
    TypeIdMissing,
    #[error("No type pointer found for type id")]
    TypePtrMissing,
}