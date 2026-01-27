use async_trait::async_trait;
use crate::domain::channel::{Channel, ChannelId};
use crate::port::db::{RepositoryError, Transaction};

#[async_trait]
pub trait ChannelRepository: Send + Sync {

    async fn insert(
        &self,
        transaction: &dyn Transaction,
        channel: &Channel,
    ) -> Result<(), RepositoryError>;

    async fn load_all(
        &self,
        transaction: &dyn Transaction,
    ) -> Result<Vec<Channel>, RepositoryError>;

    async fn load_by_id(
        &self,
        transaction: &dyn Transaction,
        channel_id: &ChannelId,
    ) -> Result<Option<Channel>, RepositoryError>;
}