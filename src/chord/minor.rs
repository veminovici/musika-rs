use super::{Chord, InnerChord};
use crate::Note;
use std::fmt::{Debug, Display, LowerHex, UpperHex};

pub struct Minor(InnerChord);

impl Minor {
    pub fn new(root: Note) -> Self {
        let steps = [3, 4];
        Self(InnerChord::with_steps(root, steps.into_iter()))
    }
}

impl From<Note> for Minor {
    fn from(root: Note) -> Self {
        Minor::new(root)
    }
}

impl Display for Minor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}min", self.root())
    }
}

impl Debug for Minor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chord: Min, {:?}, {:?}", self.root(), self.0)
    }
}

impl UpperHex for Minor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}Min {:X}", self.root(), self.0)
    }
}

impl LowerHex for Minor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}Min {:x}", self.root(), self.0)
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
        self.0.root()
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::C;

    #[test]
    fn new() {
        let chord = Minor::new(C);

        assert_eq!(chord.root(), &C);
        assert_eq!(chord.len(), 3);
        assert!(!chord.is_empty());
    }
}
