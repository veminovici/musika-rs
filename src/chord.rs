use crate::{Note, Notes, Tone};
use std::{
    fmt::{Debug, Display},
    ops::Index,
};

pub struct Chord(Notes);

impl Chord {
    fn with_stepper(root: &Note, steps: impl Iterator<Item = Tone>) -> Self {
        let notes = Notes::with_stepper(root, steps);
        Self(notes)
    }

    pub fn major(root: &Note) -> Self {
        let steps = [Tone(4), Tone(3)];
        Self::with_stepper(root, steps.into_iter())
    }

    pub fn minor(root: &Note) -> Self {
        let steps = [Tone(3), Tone(4)];
        Self::with_stepper(root, steps.into_iter())
    }

    pub fn diminished(root: &Note) -> Self {
        let steps = [Tone(3), Tone(3)];
        Self::with_stepper(root, steps.into_iter())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn root(&self) -> &Note {
        self.0.root()
    }
}

impl IntoIterator for Chord {
    type Item = Note;

    type IntoIter = <Vec<Note> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl AsRef<Notes> for Chord {
    fn as_ref(&self) -> &Notes {
        &self.0
    }
}

impl Debug for Chord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Index<usize> for Chord {
    type Output = Note;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl<N> From<N> for Chord
where
    N: Iterator<Item = Note>,
{
    fn from(notes: N) -> Self {
        Self(notes.into())
    }
}

impl From<Notes> for Chord {
    fn from(notes: Notes) -> Self {
        Self(notes)
    }
}

pub enum Chords {
    Major3(Chord),
    Minor3(Chord),
    Diminished3(Chord),
}

impl Chords {
    pub fn major(root: &Note) -> Self {
        let chord = Chord::major(root);
        Self::Major3(chord)
    }

    pub fn minor(root: &Note) -> Self {
        let chord = Chord::minor(root);
        Self::Minor3(chord)
    }

    pub fn diminished(root: &Note) -> Self {
        let chord = Chord::diminished(root);
        Self::Diminished3(chord)
    }
}

impl From<Chords> for Chord {
    fn from(chord: Chords) -> Self {
        match chord {
            Chords::Major3(c) => c,
            Chords::Minor3(c) => c,
            Chords::Diminished3(c) => c,
        }
    }
}

impl Display for Chords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Chords::Major3(c) => write!(f, "{}", c.root()),
            Chords::Minor3(c) => write!(f, "{}min", c.root()),
            Chords::Diminished3(c) => write!(f, "{}dim", c.root()),
        }
    }
}

impl Debug for Chords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Major3(chord) => write!(f, "{} {:?}", chord.root(), chord),
            Self::Minor3(chord) => write!(f, "{}min {:?}", chord.root(), chord),
            Self::Diminished3(chord) => write!(f, "{}dim {:?}", chord.root(), chord),
        }
    }
}

impl IntoIterator for Chords {
    type Item = Note;

    type IntoIter = <Chord as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Chords::Major3(chord) => chord.into_iter(),
            Chords::Minor3(chord) => chord.into_iter(),
            Chords::Diminished3(chord) => chord.into_iter(),
        }
    }
}

impl Index<usize> for Chords {
    type Output = Note;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Chords::Major3(chord) => chord.index(index),
            Chords::Minor3(chord) => chord.index(index),
            Chords::Diminished3(chord) => chord.index(index),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{A, B, C, D, E, F, G, O};

    #[test]
    fn major() {
        let chord = Chord::major(&C);
        assert_eq!(chord.root(), &C);
        assert_eq!(chord.len(), 3);

        let mut iter = chord.into_iter();
        assert_eq!(iter.next(), Some(C));
        assert_eq!(iter.next(), Some(E));
        assert_eq!(iter.next(), Some(G));
        assert!(iter.next().is_none());
    }

    #[test]
    fn minor() {
        let chord = Chord::minor(&A);
        assert_eq!(chord.root(), &A);
        assert_eq!(chord.len(), 3);

        let mut iter = chord.into_iter();
        assert_eq!(iter.next(), Some(A));
        assert_eq!(iter.next(), Some(C + O));
        assert_eq!(iter.next(), Some(E + O));
        assert!(iter.next().is_none());
    }

    #[test]
    fn diminshed() {
        let chord = Chord::diminished(&B);
        assert_eq!(chord.root(), &B);
        assert_eq!(chord.len(), 3);

        let mut iter = chord.into_iter();
        assert_eq!(iter.next(), Some(B));
        assert_eq!(iter.next(), Some(D + O));
        assert_eq!(iter.next(), Some(F + O));
        assert!(iter.next().is_none());
    }

    #[test]
    fn chords() {
        let cmaj = Chords::major(&C);
        assert_eq!(cmaj.to_string(), "C");

        let amin = Chords::minor(&A);
        assert_eq!(amin.to_string(), "Amin");

        let bdim = Chords::diminished(&B);
        assert_eq!(bdim.to_string(), "Bdim");
    }
}
