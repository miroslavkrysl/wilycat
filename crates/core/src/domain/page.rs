use std::fmt::Debug;
use std::hash::Hash;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SeekPage<T> {
    items: Vec<T>,
    has_more: bool,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SeekDirection {
    Next,
    Previous,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SeekQuery<Cursor>
{
    cursor_id: Option<Cursor>,
    direction: SeekDirection,
    limit: usize,
}
