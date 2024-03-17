use super::{Chord, InnerChord};
use crate::Note;
use std::fmt::{Debug, Display, LowerHex, UpperHex};

/// Implements the **diminished 7** chord.
/// [Cdim7 chord](https://www.pianochord.org/c-dim7.html) (C - Eb - Gb - A)
pub struct Diminished7(InnerChord);

impl Diminished7 {
    pub fn new(root: Note) -> Self {
        let steps = [3, 3, 3];
        Self(InnerChord::with_steps(root, steps.into_iter()))
    }
}

impl From<Note> for Diminished7 {
    fn from(root: Note) -> Self {
        Diminished7::new(root)
    }
}

impl Display for Diminished7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}dim7", self.root())
    }
}

impl Debug for Diminished7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chord: dim7, {:?}, {:?}", self.root(), self.0)
    }
}

impl UpperHex for Diminished7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}dim7 {:X}", self.root(), self.0)
    }
}

impl LowerHex for Diminished7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}dim7 {:x}", self.root(), self.0)
    }
}

impl IntoIterator for Diminished7 {
    type Item = Note;

    type IntoIter = <Vec<Note> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Chord for Diminished7 {
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
    use crate::{A, C, D_SHARP, F_SHARP};

    #[test]
    fn new() {
        let chord = Diminished7::new(C);

        assert_eq!(chord.root(), &C);
        assert_eq!(chord.len(), 4);
        assert!(!chord.is_empty());

        let notes = chord.into_iter().map(|n| n.base()).collect::<Vec<_>>();
        assert_eq!(notes, vec![C, D_SHARP, F_SHARP, A]);
    }

    #[test]
    fn show() {
        let chord = Diminished7::new(C);
        assert_eq!(format!("{chord}"), "Cdim7");
        assert_eq!(
            format!("{chord:?}"),
            "Chord: dim7, C4:C:0, [C4:C:0, C4:D#:3, C4:F#:6, C5:A:9]"
        );
        assert_eq!(format!("{chord:X}"), "Cdim7 [C, D#, F#, A]");
        assert_eq!(format!("{chord:x}"), "Cdim7 [C, Eb, Gb, A]");
    }
}
