use super::Chord;
use crate::{
    chord::{to_debug, to_lower_hex, to_upper_hex},
    Note, NoteStepperIterator,
};
use std::fmt::{Debug, Display, LowerHex, UpperHex};

pub struct Minor(Vec<Note>);

impl Minor {
    pub fn new(root: Note) -> Self {
        let steps = [3, 4];
        let notes = NoteStepperIterator::new(root, steps.into_iter()).collect::<Vec<_>>();
        Self(notes)
    }
}

impl From<Note> for Minor {
    fn from(root: Note) -> Self {
        Minor::new(root)
    }
}

impl Display for Minor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}min", self.root())
    }
}

impl Debug for Minor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let notes = to_debug(self.0.iter());
        write!(f, "Chord: Min, {:?}, [{notes}]", self.root())
    }
}

impl UpperHex for Minor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let notes = to_upper_hex(self.0.iter());
        write!(f, "{self} [{notes}]")
    }
}

impl LowerHex for Minor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let notes = to_lower_hex(self.0.iter());
        write!(f, "{self} [{notes}]")
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
    use super::*;
    use crate::C;

    #[test]
    fn new() {
        let chord = Minor::new(C);

        assert_eq!(chord.root(), &C);
        assert_eq!(chord.len(), 3);
        assert!(!chord.is_empty());
    }
}