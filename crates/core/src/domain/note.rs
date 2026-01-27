use crate::domain::id::{validate_uuid_v7, IdError};
use serde::{Deserialize, Serialize};
use time::UtcDateTime;
use uuid::Uuid;

/// The note ID wrapper. The internal UUID is version 7.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NoteId {
    id: Uuid,
}

impl NoteId {
    /// Creates a new note ID from the given UUID.
    ///
    /// The UUID must be version 7 or else it returns an error.
    pub fn new(id: Uuid) -> Result<Self, IdError> {
        validate_uuid_v7(id)?;
        Ok(Self { id })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NoteText {
    text: String,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Note {
    id: NoteId,
    created_at: UtcDateTime,
    text: String,
}

impl Note {
    pub fn new(id: NoteId, created_at: UtcDateTime, text: String) -> Self {
        Self {
            id,
            created_at,
            text,
        }
    }
}
