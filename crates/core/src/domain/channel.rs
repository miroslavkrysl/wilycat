use crate::domain::id::{validate_uuid_v7, IdError};
use serde::{Deserialize, Serialize};
use time::UtcDateTime;
use uuid::Uuid;

/// The channel ID wrapper. The internal UUID is version 7.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ChannelId {
    id: Uuid,
}

impl ChannelId {
    /// Creates a new note ID from the given UUID.
    ///
    /// The UUID must be version 7 or else it returns an error.
    pub fn new(id: Uuid) -> Result<Self, IdError> {
        validate_uuid_v7(id)?;
        Ok(Self { id })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Channel {
    id: ChannelId,
    created_at: UtcDateTime,
    name: ChannelName,
}

impl Channel {
    pub fn new(id: ChannelId, created_at: UtcDateTime, name: ChannelName) -> Self {
        Self {
            id,
            created_at,
            name,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct ChannelName {
    string: String,
}

impl ChannelName {
    pub fn new(string: String) -> Self {
        Self { string }
    }
}
