use super::Chord;
use crate::{
    chord::{to_debug, to_lower_hex, to_upper_hex},
    Note, NoteStepperIterator,
};
use std::fmt::{Debug, Display, LowerHex, UpperHex};

pub struct Major7(Vec<Note>);

impl Major7 {
    pub fn new(root: Note) -> Self {
        let steps = [4, 3, 4];
        let notes = NoteStepperIterator::new(root, steps.into_iter()).collect::<Vec<_>>();
        Self(notes)
    }
}

impl From<Note> for Major7 {
    fn from(root: Note) -> Self {
        Major7::new(root)
    }
}

impl Display for Major7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}7", self.root())
    }
}

impl Debug for Major7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let notes = to_debug(self.0.iter());
        write!(f, "Chord: Maj7, {:?}, [{notes}]", self.root())
    }
}

impl UpperHex for Major7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let notes = to_upper_hex(self.0.iter());
        write!(f, "{self} [{notes}]")
    }
}

impl LowerHex for Major7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let notes = to_lower_hex(self.0.iter());
        write!(f, "{self} [{notes}]")
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
        assert_eq!(format!("{chord}"), "C7");
        assert_eq!(
            format!("{chord:?}"),
            "Chord: Maj7, C4:C:0, [C4:C:0, C4:E:4, C4:G:7, C5:B:11]"
        );
        assert_eq!(format!("{chord:X}"), "C7 [C, E, G, B]");
        assert_eq!(format!("{chord:x}"), "C7 [C, E, G, B]");
    }
}
