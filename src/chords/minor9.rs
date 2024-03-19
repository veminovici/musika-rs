use super::{Chord, InnerChord};
use crate::Note;
use std::fmt::{Debug, Display, LowerHex, UpperHex};

/// Implements the **minor 7** chord.
/// [Cm9 chord](https://www.pianochord.org/cm9.html) (C - Eb - G - Bb - D)
pub struct Minor9(InnerChord);

impl Minor9 {
    pub fn new(root: Note) -> Self {
        let steps = [3, 4, 3, 4];
        Self(InnerChord::with_steps(root, steps.into_iter()))
    }
}

impl From<Note> for Minor9 {
    fn from(root: Note) -> Self {
        Minor9::new(root)
    }
}

impl Display for Minor9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}m9", self.root())
    }
}

impl Debug for Minor9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chord: m9, {:?}, {:?}", self.root(), self.0)
    }
}

impl UpperHex for Minor9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}m9 {:X}", self.root(), self.0)
    }
}

impl LowerHex for Minor9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}m9 {:x}", self.root(), self.0)
    }
}

impl IntoIterator for Minor9 {
    type Item = Note;

    type IntoIter = <Vec<Note> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Chord for Minor9 {
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
    use crate::{A_SHARP, C, D, D_SHARP, G};

    #[test]
    fn new() {
        let chord = Minor9::new(C);

        assert_eq!(chord.root(), &C);
        assert_eq!(chord.len(), 5);
        assert!(!chord.is_empty());

        let notes = chord.into_iter().map(|n| n.base()).collect::<Vec<_>>();
        assert_eq!(notes, vec![C, D_SHARP, G, A_SHARP, D]);
    }

    #[test]
    fn show() {
        let chord = Minor9::new(C);
        assert_eq!(format!("{chord}"), "Cm9");
        assert_eq!(
            format!("{chord:?}"),
            "Chord: m9, C4:C:0, [C4:C:0, C4:D#:3, C4:G:7, C5:A#:10, C5:D:14]"
        );
        assert_eq!(format!("{chord:X}"), "Cm9 [C, D#, G, A#, D]");
        assert_eq!(format!("{chord:x}"), "Cm9 [C, Eb, G, Bb, D]");
    }
}
