use crate::domain::note::{Note, NoteId};
use crate::domain::page::{SeekPage, SeekQuery};

pub trait LoadNotesPageUseCase {

    fn load(query: SeekQuery<NoteId>) -> SeekPage<Note>;
}

