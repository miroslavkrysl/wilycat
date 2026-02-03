//! The `TransactionError` enum represents a set of errors that can occur while managing database transactions.

use async_trait::async_trait;
use downcast_rs::{impl_downcast, DowncastSync};
use mockall::automock;
use std::fmt::Display;
use thiserror::Error;

pub mod channel;
pub mod note;

#[async_trait]
pub trait Database: Send + Sync {
    /// Execute the given future within a transaction.
    ///
    /// If the future completes successfully, the transaction is committed.
    /// If the future returns an error, the transaction is rolled back.
    async fn connect(&self) -> Result<Box<dyn Connection>, anyhow::Error>;
}

#[automock]
#[async_trait]
pub trait Connection: Send {
    /// Begin a new transaction
    async fn begin_transaction(&mut self) -> Result<Box<dyn Transaction>, anyhow::Error>;
}

#[automock]
#[async_trait]
pub trait Transaction: Send + DowncastSync {
    /// Commit the transaction.
    async fn commit(self: Box<Self>) -> Result<(), anyhow::Error>;

    /// Roll back the transaction.
    async fn roll_back(self: Box<Self>) -> Result<(), anyhow::Error>;
}

impl_downcast!(sync Transaction);

/// Helper function to run an operation within a transaction.
pub async fn run_in_transaction<F, R, E>(
    conn: &mut dyn Connection,
    operation: F,
) -> Result<R, TransactionError<E>>
where
    F: for<'a> AsyncFnOnce(&'a mut dyn Transaction) -> Result<R, E>,
    E: Display,
{
    let mut tx = conn
        .begin_transaction()
        .await
        .map_err(|e| TransactionError::Begin(e))?;

    match operation(&mut *tx).await {
        Ok(result) => {
            tx.commit().await.map_err(|e| TransactionError::Commit(e))?;
            Ok(result)
        }
        Err(operation_error) => match tx.roll_back().await {
            Ok(_) => Err(TransactionError::Operation(operation_error)),
            Err(rollback_error) => Err(TransactionError::Rollback {
                operation_error,
                rollback_error,
            }),
        },
    }
}

#[derive(Debug, Error)]
pub enum TransactionError<E: Display> {
    #[error("Transaction begin error: {0}")]
    Begin(#[source] anyhow::Error),

    #[error(transparent)]
    Operation(#[from] E),

    #[error("Transaction commit error: {0}")]
    Commit(#[source] anyhow::Error),

    #[error("Transaction rollback error: {rollback_error} [operation error: {operation_error}")]
    Rollback {
        operation_error: E,
        #[source]
        rollback_error: anyhow::Error,
    },
}

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Operation error: {0}")]
    Operation(#[source] anyhow::Error),

    #[error("Database error: {0}")]
    Database(#[source] anyhow::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    mod run_in_transaction {
        use super::*;
        use anyhow::anyhow;

        #[tokio::test]
        async fn operation_success() {
            let mut connection = MockConnection::new();
            let mut transaction = Box::new(MockTransaction::new());

            transaction.expect_commit().times(1).returning(|| Ok(()));

            connection
                .expect_begin_transaction()
                .times(1)
                .return_once(move || Ok(transaction));

            let result: Result<i32, TransactionError<anyhow::Error>> =
                run_in_transaction(&mut connection, async |_| Ok(42i32)).await;

            assert_eq!(result.unwrap(), 42i32);
        }

        #[tokio::test]
        async fn begin_failure() {
            let mut connection = MockConnection::new();

            connection
                .expect_begin_transaction()
                .times(1)
                .return_once(move || Err(anyhow!("begin error")));

            let result: Result<i32, TransactionError<anyhow::Error>> =
                run_in_transaction(&mut connection, async |_| Ok(42i32)).await;

            if let Err(TransactionError::Begin(error)) = result {
                assert_eq!(error.to_string(), "begin error");
            } else {
                panic!("Expected begin error");
            }
        }

        #[tokio::test]
        async fn operation_failure() {
            let mut connection = MockConnection::new();
            let mut transaction = Box::new(MockTransaction::new());

            transaction.expect_roll_back().times(1).returning(|| Ok(()));

            connection
                .expect_begin_transaction()
                .times(1)
                .return_once(move || Ok(transaction));

            let result: Result<i32, TransactionError<anyhow::Error>> =
                run_in_transaction(&mut connection, async |_| Err(anyhow!("operation error")))
                    .await;

            if let Err(TransactionError::Operation(error)) = result {
                assert_eq!(error.to_string(), "operation error");
            } else {
                panic!("Expected operation error");
            }
        }

        #[tokio::test]
        async fn rollback_failure() {
            let mut connection = MockConnection::new();
            let mut transaction = Box::new(MockTransaction::new());

            transaction
                .expect_roll_back()
                .times(1)
                .returning(|| Err(anyhow!("rollback error")));

            connection
                .expect_begin_transaction()
                .times(1)
                .return_once(move || Ok(transaction));

            let result: Result<i32, TransactionError<anyhow::Error>> =
                run_in_transaction(&mut connection, async |_| Err(anyhow!("operation error")))
                    .await;

            if let Err(TransactionError::Rollback {
                operation_error,
                rollback_error,
            }) = result
            {
                assert_eq!(operation_error.to_string(), "operation error");
                assert_eq!(rollback_error.to_string(), "rollback error");
            } else {
                panic!("Expected rollback error");
            }
        }

        #[tokio::test]
        async fn commit_failure() {
            let mut connection = MockConnection::new();
            let mut transaction = Box::new(MockTransaction::new());

            transaction
                .expect_commit()
                .times(1)
                .returning(|| Err(anyhow!("commit error")));

            connection
                .expect_begin_transaction()
                .times(1)
                .return_once(move || Ok(transaction));

            let result: Result<i32, TransactionError<anyhow::Error>> =
                run_in_transaction(&mut connection, async |_| Ok(42i32)).await;

            if let Err(TransactionError::Commit(error)) = result {
                assert_eq!(error.to_string(), "commit error");
            } else {
                panic!("Expected commit error");
            }
        }
    }
}
