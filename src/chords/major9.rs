use super::{Chord, InnerChord};
use crate::Note;
use std::fmt::{Debug, Display, LowerHex, UpperHex};

/// Implements the **major 7** chord.
/// [Cmaj9 chord](https://www.pianochord.org/cmaj9.html) (C - E - G - B)
pub struct Major9(InnerChord);

impl Major9 {
    pub fn new(root: Note) -> Self {
        let steps = [4, 3, 4, 3];
        Self(InnerChord::with_steps(root, steps.into_iter()))
    }
}

impl From<Note> for Major9 {
    fn from(root: Note) -> Self {
        Major9::new(root)
    }
}

impl Display for Major9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}maj9", self.root())
    }
}

impl Debug for Major9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chord: maj9, {:?}, {:?}", self.root(), self.0)
    }
}

impl UpperHex for Major9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}maj9 {:X}", self.root(), self.0)
    }
}

impl LowerHex for Major9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}maj9 {:x}", self.root(), self.0)
    }
}

impl IntoIterator for Major9 {
    type Item = Note;

    type IntoIter = <Vec<Note> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Chord for Major9 {
    fn root(&self) -> &Note {
        self.0.root()
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn up_one_octave(self) -> Self {
        Self(self.0.up_one_octave())
    }

    fn down_one_octave(self) -> Self {
        Self(self.0.down_one_octave())
    }
}

#[cfg(test)]
mod tests {
    use crate::{B, C, D, E, G};

    use super::*;

    #[test]
    fn new() {
        let chord = Major9::new(C);

        assert_eq!(chord.root(), &C);
        assert_eq!(chord.len(), 5);
        assert!(!chord.is_empty());

        let notes = chord.into_iter().map(|n| n.base()).collect::<Vec<_>>();
        assert_eq!(notes, vec![C, E, G, B, D]);
    }

    #[test]
    fn show() {
        let chord = Major9::new(C);
        assert_eq!(format!("{chord}"), "Cmaj9");
        assert_eq!(
            format!("{chord:?}"),
            "Chord: maj9, C4:C:0, [C4:C:0, C4:E:4, C4:G:7, C5:B:11, C5:D:14]"
        );
        assert_eq!(format!("{chord:X}"), "Cmaj9 [C, E, G, B, D]");
        assert_eq!(format!("{chord:x}"), "Cmaj9 [C, E, G, B, D]");
    }
}
