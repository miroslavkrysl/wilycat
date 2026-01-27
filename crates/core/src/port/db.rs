//! The `TransactionError` enum represents a set of errors that can occur while managing database transactions.
use async_trait::async_trait;
use thiserror::Error;

pub mod note;
mod channel;

#[derive(Error, Debug)]
pub enum TransactionError {
    #[error("Failed to begin transaction: {0}")]
    BeginFailed(#[source] anyhow::Error),

    #[error("Failed to commit transaction: {0}")]
    CommitFailed(#[source] anyhow::Error),

    #[error("Failed to rollback transaction: {0}")]
    RollbackFailed(#[source] anyhow::Error),
}

#[async_trait]
pub trait Transaction: Send {
    /// Commit the transaction.
    async fn commit(self: Box<Self>) -> Result<(), TransactionError>;

    /// Roll back the transaction.
    async fn rollback(self: Box<Self>) -> Result<(), TransactionError>;
}


#[async_trait]
pub trait TransactionManager: Send + Sync {

    /// Create a new transaction.
    async fn begin(&self) -> Result<Box<dyn Transaction>, TransactionError>;
}

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Database(#[source] anyhow::Error),

    #[error("Invalid transaction type: {0}")]
    InvalidTransaction(#[source] anyhow::Error)
}