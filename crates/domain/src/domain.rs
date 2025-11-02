use serde::{Deserialize, Serialize};
use time::UtcDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct EntryId {
    entry_id: Uuid,
    workspace_id: ChannelId,
}

impl EntryId {
    pub fn new(entry_id: Uuid, workspace_id: ChannelId) -> Self {
        Self {
            entry_id,
            workspace_id,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct ChannelId {
    id: Uuid,
}

impl ChannelId {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Entry {
    id: EntryId,
    created_at: UtcDateTime,
    content: String,
}

impl Entry {
    pub fn new(id: EntryId, created_at: UtcDateTime, content: String) -> Self {
        Self {
            id,
            created_at,
            content,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Channel {
    id: EntryId,
    created_at: UtcDateTime,
    name: ChannelName,
}

impl Channel {
    pub fn new(id: EntryId, created_at: UtcDateTime, name: ChannelName) -> Self {
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
