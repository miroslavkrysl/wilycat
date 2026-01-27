use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum IdError {
    #[error("UUID must be version 7, got version {0}")]
    InvalidVersion(usize),
}

pub fn validate_uuid_v7(uuid: Uuid) -> Result<(), IdError> {
    let version = uuid.get_version_num();

    if version != 7 {
        Err(IdError::InvalidVersion(version))
    } else {
        Ok(())
    }
}
