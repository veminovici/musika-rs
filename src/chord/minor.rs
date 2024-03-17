use super::{Chord, InnerChord};
use crate::Note;
use std::fmt::{Debug, Display, LowerHex, UpperHex};

/// Implements the **minor** chord.
/// [Cm chord](https://www.pianochord.org/cm.html) (C - Eb - G)
pub struct Minor(InnerChord);

impl Minor {
    pub fn new(root: Note) -> Self {
        let steps = [3, 4];
        Self(InnerChord::with_steps(root, steps.into_iter()))
    }
}

impl From<Note> for Minor {
    fn from(root: Note) -> Self {
        Minor::new(root)
    }
}

impl Display for Minor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}m", self.root())
    }
}

impl Debug for Minor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chord: m, {:?}, {:?}", self.root(), self.0)
    }
}

impl UpperHex for Minor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}m {:X}", self.root(), self.0)
    }
}

impl LowerHex for Minor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}m {:x}", self.root(), self.0)
    }
}

impl IntoIterator for Minor {
    type Item = Note;

    type IntoIter = <Vec<Note> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Chord for Minor {
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
    use crate::{C, D_SHARP, G};

    #[test]
    fn new() {
        let chord = Minor::new(C);

        assert_eq!(chord.root(), &C);
        assert_eq!(chord.len(), 3);
        assert!(!chord.is_empty());

        let notes = chord.into_iter().collect::<Vec<_>>();
        assert_eq!(notes, vec![C, D_SHARP, G]);
    }

    #[test]
    fn show() {
        let chord = Minor::new(C);
        assert_eq!(format!("{chord}"), "Cm");
        assert_eq!(
            format!("{chord:?}"),
            "Chord: m, C4:C:0, [C4:C:0, C4:D#:3, C4:G:7]"
        );
        assert_eq!(format!("{chord:X}"), "Cm [C, D#, G]");
        assert_eq!(format!("{chord:x}"), "Cm [C, Eb, G]");
    }
}
