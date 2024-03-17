use super::{Chord, InnerChord};
use crate::Note;
use std::fmt::{Debug, Display, LowerHex, UpperHex};

/// Implements the **major 7** chord.
/// [C7 chord](https://www.pianochord.org/cmaj7.html) (C - E - G - B)
pub struct Major7(InnerChord);

impl Major7 {
    pub fn new(root: Note) -> Self {
        let steps = [4, 3, 4];
        Self(InnerChord::with_steps(root, steps.into_iter()))
    }
}

impl From<Note> for Major7 {
    fn from(root: Note) -> Self {
        Major7::new(root)
    }
}

impl Display for Major7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}maj7", self.root())
    }
}

impl Debug for Major7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chord: maj7, {:?}, {:?}", self.root(), self.0)
    }
}

impl UpperHex for Major7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}maj7 {:X}", self.root(), self.0)
    }
}

impl LowerHex for Major7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}maj7 {:x}", self.root(), self.0)
    }
}

impl IntoIterator for Major7 {
    type Item = Note;

    type IntoIter = <Vec<Note> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Chord for Major7 {
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
    use crate::{B, C, E, G};

    use super::*;

    #[test]
    fn new() {
        let chord = Major7::new(C);

        assert_eq!(chord.root(), &C);
        assert_eq!(chord.len(), 4);
        assert!(!chord.is_empty());

        let notes = chord.into_iter().map(|n| n.base()).collect::<Vec<_>>();
        assert_eq!(notes, vec![C, E, G, B]);
    }

    #[test]
    fn show() {
        let chord = Major7::new(C);
        assert_eq!(format!("{chord}"), "Cmaj7");
        assert_eq!(
            format!("{chord:?}"),
            "Chord: maj7, C4:C:0, [C4:C:0, C4:E:4, C4:G:7, C5:B:11]"
        );
        assert_eq!(format!("{chord:X}"), "Cmaj7 [C, E, G, B]");
        assert_eq!(format!("{chord:x}"), "Cmaj7 [C, E, G, B]");
    }
}
