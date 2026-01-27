use crate::domain::channel::ChannelId;
use crate::domain::note::{Note, NoteId};
use crate::port::db::{RepositoryError, Transaction};
use async_trait::async_trait;

#[async_trait]
pub trait NoteRepository: Send + Sync {
    async fn insert(
        &self,
        transaction: &dyn Transaction,
        note: Note,
    ) -> Result<(), RepositoryError>;

    async fn load_by_id(
        &self,
        transaction: &dyn Transaction,
        note_id: NoteId,
    ) -> Result<Option<Note>, RepositoryError>;

    async fn load_after_id(
        &self,
        transaction: &dyn Transaction,
        channel_id: &ChannelId,
        after: Option<&NoteId>,
        limit: usize,
    ) -> Result<Vec<Note>, RepositoryError>;

    async fn load_before_id(
        &self,
        transaction: &dyn Transaction,
        channel_id: &ChannelId,
        after: Option<&NoteId>,
        limit: usize,
    ) -> Result<Vec<Note>, RepositoryError>;
}
