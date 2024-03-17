use super::{Chord, InnerChord};
use crate::Note;
use std::fmt::{Debug, Display, LowerHex, UpperHex};

/// Implements the **diminished** chord.
/// [Cdim chord](https://www.pianochord.org/c-dim.html) (C - Eb - Gb)
pub struct Diminished(InnerChord);

impl Diminished {
    pub fn new(root: Note) -> Self {
        let steps = [3, 3];
        Self(InnerChord::with_steps(root, steps.into_iter()))
    }
}

impl From<Note> for Diminished {
    fn from(root: Note) -> Self {
        Diminished::new(root)
    }
}

impl Display for Diminished {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}dim7", self.root())
    }
}

impl Debug for Diminished {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chord: dim7, {:?}, {:?}", self.root(), self.0)
    }
}

impl UpperHex for Diminished {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}dim7 {:X}", self.root(), self.0)
    }
}

impl LowerHex for Diminished {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}dim7 {:x}", self.root(), self.0)
    }
}

impl IntoIterator for Diminished {
    type Item = Note;

    type IntoIter = <Vec<Note> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Chord for Diminished {
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
    use crate::{C, D_SHARP, F_SHARP};

    #[test]
    fn new() {
        let chord = Diminished::new(C);

        assert_eq!(chord.root(), &C);
        assert_eq!(chord.len(), 3);
        assert!(!chord.is_empty());

        let notes = chord.into_iter().map(|n| n.base()).collect::<Vec<_>>();
        assert_eq!(notes, vec![C, D_SHARP, F_SHARP]);
    }

    #[test]
    fn show() {
        let chord = Diminished::new(C);
        assert_eq!(format!("{chord}"), "Cdim7");
        assert_eq!(
            format!("{chord:?}"),
            "Chord: dim7, C4:C:0, [C4:C:0, C4:D#:3, C4:F#:6]"
        );
        assert_eq!(format!("{chord:X}"), "Cdim7 [C, D#, F#]");
        assert_eq!(format!("{chord:x}"), "Cdim7 [C, Eb, Gb]");
    }
}
