use super::Chord;
use crate::{
    chord::{to_debug, to_lower_hex, to_upper_hex},
    Note, NoteStepperIterator,
};
use std::fmt::{Debug, Display, LowerHex, UpperHex};

pub struct Major(Vec<Note>);

impl Major {
    pub fn new(root: Note) -> Self {
        let steps = [4, 3];
        let notes = NoteStepperIterator::new(root, steps.into_iter()).collect::<Vec<_>>();
        Self(notes)
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
        let notes = to_debug(self.0.iter());
        write!(f, "Chord: Maj, {:?}, [{notes}]", self.root())
    }
}

impl UpperHex for Major {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let notes = to_upper_hex(self.0.iter());
        write!(f, "{self} [{notes}]")
    }
}

impl LowerHex for Major {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let notes = to_lower_hex(self.0.iter());
        write!(f, "{self} [{notes}]")
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
        &self.0[0]
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
