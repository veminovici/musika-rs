use std::{fmt::Debug, ops::Index};

use crate::{Note, Notes, H, W};

pub enum Scale {
    Major(Notes),
}

impl AsRef<Notes> for Scale {
    fn as_ref(&self) -> &Notes {
        match self {
            Scale::Major(notes) => notes,
        }
    }
}

impl Scale {
    pub fn major(tonic: &Note) -> Self {
        let steps = [W, W, H, W, W, W, H];
        let notes = Notes::with_stepper(tonic, steps.into_iter());
        Self::Major(notes)
    }

    pub fn len(&self) -> usize {
        self.as_ref().len()
    }

    pub fn is_empty(&self) -> bool {
        self.as_ref().is_empty()
    }

    pub fn tonic(&self) -> &Note {
        self.as_ref().root()
    }
}

impl Debug for Scale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.as_ref())
    }
}

impl IntoIterator for Scale {
    type Item = Note;

    type IntoIter = <Vec<Note> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Scale::Major(notes) => notes.into_iter(),
        }
    }
}

impl Index<usize> for Scale {
    type Output = Note;

    fn index(&self, index: usize) -> &Self::Output {
        self.as_ref().index(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{A, B, C, C5, D, E, F, G};

    #[test]
    fn major() {
        let scale = Scale::major(&C);
        assert_eq!(scale.tonic(), &C);
        assert_eq!(scale.len(), 8);

        let mut iter = scale.into_iter();
        assert_eq!(iter.next(), Some(C));
        assert_eq!(iter.next(), Some(D));
        assert_eq!(iter.next(), Some(E));
        assert_eq!(iter.next(), Some(F));
        assert_eq!(iter.next(), Some(G));
        assert_eq!(iter.next(), Some(A));
        assert_eq!(iter.next(), Some(B));

        let note = iter.next().unwrap();
        assert_eq!(note.octave(), &C5);
        assert_eq!(note.note(), 0);

        assert!(iter.next().is_none());
    }
}
