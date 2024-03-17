mod diminished;
mod diminished7;
mod dominant7;
mod major;
mod major7;
mod minor;
mod minor7;
mod minor7b5;

pub use diminished::*;
pub use diminished7::*;
pub use dominant7::*;
pub use major::*;
pub use major7::*;
pub use minor::*;
pub use minor7::*;
pub use minor7b5::*;

use crate::{Note, NoteStepperIterator, Tone, OCTAVE};
use std::fmt::{Debug, Display, LowerHex, UpperHex};

/// The chord behavior. The chord always has to be
/// convertable to a sequence of [`Note`] notes.
pub trait Chord: IntoIterator<Item = Note> {
    /// Returns the root of the chord
    fn root(&self) -> &Note;

    /// Returns the number of notes in the chord
    fn len(&self) -> usize;

    /// Determines if the chord is empty.
    fn is_empty(&self) -> bool;

    /// Moves the chord one octave up.
    fn up_one_octave(self) -> Self;

    /// Move the chord one octave down.
    fn down_one_octave(self) -> Self;
}

pub(crate) struct InnerChord(Vec<Note>);

impl InnerChord {
    fn new(n: impl Iterator<Item = Note>) -> Self {
        Self(n.collect())
    }

    pub(crate) fn with_steps<T>(root: Note, steps: impl Iterator<Item = T>) -> Self
    where
        Tone: From<T>,
    {
        let notes = NoteStepperIterator::new(root, steps.into_iter());
        Self::new(notes)
    }

    fn notes_debug(&self) -> String {
        self.0
            .iter()
            .map(|n| format!("{n:?}"))
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn notes_upper_hex(&self) -> String {
        self.0
            .iter()
            .map(|n| format!("{n:X}"))
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn notes_lower_hex(&self) -> String {
        self.0
            .iter()
            .map(|n| format!("{n:x}"))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

impl Display for InnerChord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root())
    }
}

impl Debug for InnerChord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let notes = self.notes_debug();
        write!(f, "[{notes}]")
    }
}

impl UpperHex for InnerChord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let notes = self.notes_upper_hex();
        write!(f, "[{notes}]")
    }
}

impl LowerHex for InnerChord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let notes = self.notes_lower_hex();
        write!(f, "[{notes}]")
    }
}

impl Chord for InnerChord {
    fn root(&self) -> &Note {
        &self.0[0]
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Moves the chord one octave up.
    fn up_one_octave(self) -> Self {
        let notes = self.into_iter().map(|n| n + OCTAVE);
        Self::new(notes)
    }

    fn down_one_octave(self) -> Self {
        let notes = self.into_iter().map(|n| n - OCTAVE);
        Self::new(notes)
    }
}

impl IntoIterator for InnerChord {
    type Item = Note;

    type IntoIter = <Vec<Note> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::{A, C, D, E, F, G};

    use super::*;

    #[test]
    fn move_up() {
        let chord1 = InnerChord::new([C, D, E, F, G, A].into_iter());
        let chord2 = InnerChord::new([C, D, E, F, G, A].into_iter()).up_one_octave();

        let notes1 = chord1.into_iter().map(|n| n.base()).collect::<Vec<_>>();
        let notes2 = chord2.into_iter().map(|n| n.base()).collect::<Vec<_>>();
        assert_eq!(notes1, notes2)
    }

    #[test]
    fn move_down() {
        let chord1 = InnerChord::new([C, D, E, F, G, A].into_iter());
        let chord2 = InnerChord::new([C, D, E, F, G, A].into_iter()).down_one_octave();

        let notes1 = chord1.into_iter().map(|n| n.base()).collect::<Vec<_>>();
        let notes2 = chord2.into_iter().map(|n| n.base()).collect::<Vec<_>>();
        assert_eq!(notes1, notes2)
    }
}
