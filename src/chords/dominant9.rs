use super::{Chord, InnerChord};
use crate::Note;
use std::fmt::{Debug, Display, LowerHex, UpperHex};

/// Implements the **dominant 9** chord.
/// [C9 chord](https://www.pianochord.org/c-extended.html) (C - E - G - Bb - D)
pub struct Dominant9(InnerChord);

impl Dominant9 {
    pub fn new(root: Note) -> Self {
        let steps = [4, 3, 3, 4];
        Self(InnerChord::with_steps(root, steps.into_iter()))
    }
}

impl From<Note> for Dominant9 {
    fn from(root: Note) -> Self {
        Dominant9::new(root)
    }
}

impl Display for Dominant9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}7", self.root())
    }
}

impl Debug for Dominant9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chord: 7, {:?}, {:?}", self.root(), self.0)
    }
}

impl UpperHex for Dominant9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}7 {:X}", self.root(), self.0)
    }
}

impl LowerHex for Dominant9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}7 {:x}", self.root(), self.0)
    }
}

impl IntoIterator for Dominant9 {
    type Item = Note;

    type IntoIter = <Vec<Note> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Chord for Dominant9 {
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
    use super::*;
    use crate::{A_SHARP, C, D, E, G};

    #[test]
    fn new() {
        let chord = Dominant9::new(C);

        assert_eq!(chord.root(), &C);
        assert_eq!(chord.len(), 5);
        assert!(!chord.is_empty());

        let notes = chord.into_iter().map(|n| n.base()).collect::<Vec<_>>();
        assert_eq!(notes, vec![C, E, G, A_SHARP, D]);
    }

    #[test]
    fn show() {
        let chord = Dominant9::new(C);
        assert_eq!(format!("{chord}"), "C7");
        assert_eq!(
            format!("{chord:?}"),
            "Chord: 7, C4:C:0, [C4:C:0, C4:E:4, C4:G:7, C5:A#:10, C5:D:14]"
        );
        assert_eq!(format!("{chord:X}"), "C7 [C, E, G, A#, D]");
        assert_eq!(format!("{chord:x}"), "C7 [C, E, G, Bb, D]");
    }
}
