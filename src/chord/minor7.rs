use super::{Chord, InnerChord};
use crate::Note;
use std::fmt::{Debug, Display, LowerHex, UpperHex};

/// Implements the **minor 7** chord.
/// [Cm7 chord](https://www.pianochord.org/cm7.html) (C - Eb - G - Bb)
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
        write!(f, "{}m7", self.root())
    }
}

impl Debug for Minor7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chord: m7, {:?}, {:?}", self.root(), self.0)
    }
}

impl UpperHex for Minor7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}m7 {:X}", self.root(), self.0)
    }
}

impl LowerHex for Minor7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}m7 {:x}", self.root(), self.0)
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

    #[test]
    fn show() {
        let chord = Minor7::new(C);
        assert_eq!(format!("{chord}"), "Cm7");
        assert_eq!(
            format!("{chord:?}"),
            "Chord: m7, C4:C:0, [C4:C:0, C4:D#:3, C4:G:7, C5:A#:10]"
        );
        assert_eq!(format!("{chord:X}"), "Cm7 [C, D#, G, A#]");
        assert_eq!(format!("{chord:x}"), "Cm7 [C, Eb, G, Bb]");
    }
}
