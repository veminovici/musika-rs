use crate::{Note, Notes, Tone};
use std::{
    fmt::{Debug, Display},
    ops::Index,
};

/// The chord behavior. The chord always has to be
/// convertable to a sequence of [`Note`] notes.
pub trait Chord: IntoIterator<Item = Note> {
    fn root(&self) -> &Note;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

/// The major chord based on a given root note.
pub struct Major(Notes);

impl Major {
    pub fn new(root: &Note) -> Self {
        let steps = [Tone(4), Tone(3)];
        let notes = Notes::with_stepper(root, steps.into_iter());
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

impl Chord for Major {
    fn root(&self) -> &Note {
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
        write!(f, "{}", self.root())
    }
}

impl Debug for Major {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?}", self.root(), self.0)
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

/// Minor chord based on the root note.
pub struct Minor(Notes);

impl Minor {
    pub fn new(root: &Note) -> Self {
        let steps = [Tone(3), Tone(4)];
        let notes = Notes::with_stepper(root, steps.into_iter());
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

impl Chord for Minor {
    fn root(&self) -> &Note {
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
        write!(f, "{}min", self.root())
    }
}

impl Debug for Minor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?}", self.root(), self.0)
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

/// Diminished chord based on a root note.
pub struct Diminished(Notes);

impl Diminished {
    pub fn new(root: &Note) -> Self {
        let steps = [Tone(3), Tone(3)];
        let notes = Notes::with_stepper(root, steps.into_iter());
        Self(notes)
    }
}

impl IntoIterator for Diminished {
    type Item = Note;

    type IntoIter = <Vec<Note> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Chord for Diminished {
    fn root(&self) -> &Note {
        &self.0[0]
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Display for Diminished {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}dim", self.root())
    }
}

impl Debug for Diminished {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?}", self.root(), self.0)
    }
}

impl Index<usize> for Diminished {
    type Output = Note;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl AsRef<Notes> for Diminished {
    fn as_ref(&self) -> &Notes {
        &self.0
    }
}

impl From<&Note> for Diminished {
    fn from(root: &Note) -> Self {
        Diminished::new(root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{A, B, C, D, E, F, G, O};

    #[test]
    fn major() {
        let chord = Major::from(&C);
        assert_eq!(chord.root(), &C);
        assert_eq!(chord.len(), 3);

        assert_eq!(chord.to_string(), "C");

        let mut iter = chord.into_iter();
        assert_eq!(iter.next(), Some(C));
        assert_eq!(iter.next(), Some(E));
        assert_eq!(iter.next(), Some(G));
        assert!(iter.next().is_none());
    }

    #[test]
    fn minor() {
        let chord = Minor::from(&A);
        assert_eq!(chord.root(), &A);
        assert_eq!(chord.len(), 3);

        assert_eq!(chord.to_string(), "Amin");

        let mut iter = chord.into_iter();
        assert_eq!(iter.next(), Some(A));
        assert_eq!(iter.next(), Some(C + O));
        assert_eq!(iter.next(), Some(E + O));
        assert!(iter.next().is_none());
    }

    #[test]
    fn diminshed() {
        let chord = Diminished::from(&B);
        assert_eq!(chord.root(), &B);
        assert_eq!(chord.len(), 3);

        assert_eq!(chord.to_string(), "Bdim");

        let mut iter = chord.into_iter();
        assert_eq!(iter.next(), Some(B));
        assert_eq!(iter.next(), Some(D + O));
        assert_eq!(iter.next(), Some(F + O));
        assert!(iter.next().is_none());
    }
}
