use crate::libs::repository;

#[derive(Debug, thiserror::Error)]
pub enum KCError {
    #[error(transparent)]
    Database(#[from] repository::Error),
}

pub type KCResult<T> = Result<T, KCError>;
