use super::{Chord, InnerChord};
use crate::Note;
use std::fmt::{Debug, Display, LowerHex, UpperHex};

/// Implements the **major** chord.
/// [C chord](https://www.pianochord.org/c-major.html) (C - E - G)
pub struct Major(InnerChord);

impl Major {
    pub fn new(root: Note) -> Self {
        let steps = [4, 3];
        Self(InnerChord::with_steps(root, steps.into_iter()))
    }
}

impl From<Note> for Major {
    fn from(root: Note) -> Self {
        Major::new(root)
    }
}

impl Display for Major {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root())
    }
}

impl Debug for Major {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chord: Maj, {:?}, {:?}", self.root(), self.0)
    }
}

impl UpperHex for Major {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X} {:X}", self.root(), self.0)
    }
}

impl LowerHex for Major {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x} {:x}", self.root(), self.0)
    }
}

impl IntoIterator for Major {
    type Item = Note;

    type IntoIter = <Vec<Note> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Chord for Major {
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
    use crate::{C, E, G};

    use super::*;

    #[test]
    fn new() {
        let chord = Major::new(C);

        assert_eq!(chord.root(), &C);
        assert_eq!(chord.len(), 3);
        assert!(!chord.is_empty());

        let notes = chord.into_iter().collect::<Vec<_>>();
        assert_eq!(notes, vec![C, E, G]);
    }

    #[test]
    fn show() {
        let chord = Major::new(C);
        assert_eq!(format!("{chord}"), "C");
        assert_eq!(
            format!("{chord:?}"),
            "Chord: Maj, C4:C:0, [C4:C:0, C4:E:4, C4:G:7]"
        );
        assert_eq!(format!("{chord:X}"), "C [C, E, G]");
        assert_eq!(format!("{chord:x}"), "C [C, E, G]");
    }
}
