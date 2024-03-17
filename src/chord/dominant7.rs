use super::{Chord, InnerChord};
use crate::Note;
use std::fmt::{Debug, Display, LowerHex, UpperHex};

/// Implements the **dominant 7** chord.
/// [C7 chord](https://www.pianochord.org/c7.html) (C - E - G - Bb)
pub struct Dominant7(InnerChord);

impl Dominant7 {
    pub fn new(root: Note) -> Self {
        let steps = [4, 3, 3];
        Self(InnerChord::with_steps(root, steps.into_iter()))
    }
}

impl From<Note> for Dominant7 {
    fn from(root: Note) -> Self {
        Dominant7::new(root)
    }
}

impl Display for Dominant7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}7", self.root())
    }
}

impl Debug for Dominant7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chord: 7, {:?}, {:?}", self.root(), self.0)
    }
}

impl UpperHex for Dominant7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}7 {:X}", self.root(), self.0)
    }
}

impl LowerHex for Dominant7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}7 {:x}", self.root(), self.0)
    }
}

impl IntoIterator for Dominant7 {
    type Item = Note;

    type IntoIter = <Vec<Note> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Chord for Dominant7 {
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
    use crate::{A_SHARP, C, E, G};

    #[test]
    fn new() {
        let chord = Dominant7::new(C);

        assert_eq!(chord.root(), &C);
        assert_eq!(chord.len(), 4);
        assert!(!chord.is_empty());

        let notes = chord.into_iter().map(|n| n.base()).collect::<Vec<_>>();
        assert_eq!(notes, vec![C, E, G, A_SHARP]);
    }

    #[test]
    fn show() {
        let chord = Dominant7::new(C);
        assert_eq!(format!("{chord}"), "C7");
        assert_eq!(
            format!("{chord:?}"),
            "Chord: 7, C4:C:0, [C4:C:0, C4:E:4, C4:G:7, C5:A#:10]"
        );
        assert_eq!(format!("{chord:X}"), "C7 [C, E, G, A#]");
        assert_eq!(format!("{chord:x}"), "C7 [C, E, G, Bb]");
    }
}
