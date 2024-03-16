use crate::{Note, Notes, H, W};
use std::{
    fmt::{Debug, Display},
    ops::Index,
};

/// The behavior of a scale.
pub trait Scale: IntoIterator<Item = Note> {
    fn tonic(&self) -> &Note;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

/// The major scale based on a tonic note.
pub struct Major(Notes);

impl Major {
    pub fn new(tonic: &Note) -> Self {
        let steps = [W, W, H, W, W, W, H];
        let notes = Notes::with_stepper(tonic, steps.into_iter());
        Self(notes)
    }
}

impl IntoIterator for Major {
    type Item = Note;

    type IntoIter = <Vec<Note> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Scale for Major {
    fn tonic(&self) -> &Note {
        &self.0[0]
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Display for Major {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.tonic())
    }
}

impl Debug for Major {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.tonic(), self.0)
    }
}

impl Index<usize> for Major {
    type Output = Note;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl AsRef<Notes> for Major {
    fn as_ref(&self) -> &Notes {
        &self.0
    }
}

impl From<&Note> for Major {
    fn from(root: &Note) -> Self {
        Major::new(root)
    }
}

pub struct Minor(Notes);

impl Minor {
    pub fn new(tonic: &Note) -> Self {
        let steps = [W, H, W, W, H, W, H];
        let notes = Notes::with_stepper(tonic, steps.into_iter());
        Self(notes)
    }
}

impl IntoIterator for Minor {
    type Item = Note;

    type IntoIter = <Vec<Note> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Scale for Minor {
    fn tonic(&self) -> &Note {
        &self.0[0]
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Display for Minor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}min", self.tonic())
    }
}

impl Debug for Minor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}min {}", self.tonic(), self.0)
    }
}

impl Index<usize> for Minor {
    type Output = Note;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl AsRef<Notes> for Minor {
    fn as_ref(&self) -> &Notes {
        &self.0
    }
}

impl From<&Note> for Minor {
    fn from(root: &Note) -> Self {
        Minor::new(root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{A, B, C, C5, D, E, F, G};

    #[test]
    fn major() {
        let scale = Major::new(&C);
        assert_eq!(scale.tonic(), &C);
        assert_eq!(scale.len(), 8);

        assert_eq!(scale.to_string(), "C");
        assert_eq!(format!("{scale:?}"), "C [C, D, E, F, G, A, B, C]");

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

    #[test]
    fn minor() {
        let scale = Minor::new(&C);

        assert_eq!(scale.tonic(), &C);
        assert_eq!(scale.len(), 8);

        assert_eq!(scale.to_string(), "Cmin");
    }
}
