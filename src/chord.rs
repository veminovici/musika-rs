use crate::{Note, NoteCollectionIterator, Notes, Tone};

pub struct Chord<const N: usize>(Notes<N>);

impl<const N: usize> Chord<N> {
    pub fn new(root: &Note, steps: [Tone; N]) -> Self {
        let notes = Notes::new(root, steps);
        Self(notes)
    }

    pub fn len(&self) -> usize {
        N + 1
    }

    pub fn root(&self) -> &Note {
        &self.0.root()
    }

    pub fn index(&self, idx: usize) -> Option<&Note> {
        match idx {
            0 => Some(&self.root()),
            n if n <= N => self.index(idx),
            _ => None,
        }
    }
}

impl<const N: usize> IntoIterator for Chord<N> {
    type Item = Note;

    type IntoIter = NoteCollectionIterator<N>;

    fn into_iter(self) -> Self::IntoIter {
        NoteCollectionIterator::new(self.0)
    }
}
