use super::{Chord, InnerChord};
use crate::Note;
use std::fmt::{Debug, Display, LowerHex, UpperHex};

pub struct Minor7(InnerChord);

impl Minor7 {
    pub fn new(root: Note) -> Self {
        let steps = [3, 4, 3];
        Self(InnerChord::with_steps(root, steps.into_iter()))
    }
}

impl From<Note> for Minor7 {
    fn from(root: Note) -> Self {
        Minor7::new(root)
    }
}

impl Display for Minor7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}min7", self.root())
    }
}

impl Debug for Minor7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chord: Min7, {:?}, {:?}", self.root(), self.0)
    }
}

impl UpperHex for Minor7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}Min7 {:X}", self.root(), self.0)
    }
}

impl LowerHex for Minor7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}Min {:x}", self.root(), self.0)
    }
}

impl IntoIterator for Minor7 {
    type Item = Note;

    type IntoIter = <Vec<Note> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Chord for Minor7 {
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
    use crate::{A_SHARP, C, D_SHARP, G};

    #[test]
    fn new() {
        let chord = Minor7::new(C);

        assert_eq!(chord.root(), &C);
        assert_eq!(chord.len(), 4);
        assert!(!chord.is_empty());

        let notes = chord.into_iter().map(|n| n.base()).collect::<Vec<_>>();
        assert_eq!(notes, vec![C, D_SHARP, G, A_SHARP]);
    }
}
