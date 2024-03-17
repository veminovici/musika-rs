use super::{Chord, InnerChord};
use crate::Note;
use std::fmt::{Debug, Display, LowerHex, UpperHex};

/// Implements the **minor 7b5** chord.
/// [Cm7b5 chord](https://www.pianochord.org/cm7b5.html) (C - Eb - Gb - Bb)
pub struct Minor7b5(InnerChord);

impl Minor7b5 {
    pub fn new(root: Note) -> Self {
        let steps = [3, 3, 4];
        Self(InnerChord::with_steps(root, steps.into_iter()))
    }
}

impl From<Note> for Minor7b5 {
    fn from(root: Note) -> Self {
        Minor7b5::new(root)
    }
}

impl Display for Minor7b5 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}m7b5", self.root())
    }
}

impl Debug for Minor7b5 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chord: m7b5, {:?}, {:?}", self.root(), self.0)
    }
}

impl UpperHex for Minor7b5 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}m7b5 {:X}", self.root(), self.0)
    }
}

impl LowerHex for Minor7b5 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}m7b5 {:x}", self.root(), self.0)
    }
}

impl IntoIterator for Minor7b5 {
    type Item = Note;

    type IntoIter = <Vec<Note> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Chord for Minor7b5 {
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
    use crate::{A_SHARP, C, D_SHARP, F_SHARP};

    #[test]
    fn new() {
        let chord = Minor7b5::new(C);

        assert_eq!(chord.root(), &C);
        assert_eq!(chord.len(), 4);
        assert!(!chord.is_empty());

        let notes = chord.into_iter().map(|n| n.base()).collect::<Vec<_>>();
        assert_eq!(notes, vec![C, D_SHARP, F_SHARP, A_SHARP]);
    }

    // #[test]
    // fn show() {
    //     let chord = Minor7b5::new(C);
    //     assert_eq!(format!("{chord}"), "Cm7b5");
    //     assert_eq!(
    //         format!("{chord:?}"),
    //         "Chord: m7b5, C4:C:0, [C4:C:0, C4:D#:3, C4:F#:6, C5:A#:10]"
    //     );
    //     assert_eq!(format!("{chord:X}"), "Cm7b5 [C, D#, F#, A#]");
    //     assert_eq!(format!("{chord:x}"), "Cm7b5 [C, Eb, Gb, Bb]");
    // }
}