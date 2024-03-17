mod dominant7;
mod major;
mod major7;
mod minor;

pub use dominant7::*;
pub use major::*;
pub use major7::*;
pub use minor::*;

use crate::Note;

/// The chord behavior. The chord always has to be
/// convertable to a sequence of [`Note`] notes.
pub trait Chord: std::fmt::Display + IntoIterator<Item = Note> {
    /// Returns the root of the chord
    fn root(&self) -> &Note;

    /// Returns the number of notes in the chord
    fn len(&self) -> usize;

    /// Determines if the chord is empty.
    fn is_empty(&self) -> bool;
}

pub(crate) fn to_debug<'a>(notes: impl Iterator<Item = &'a Note>) -> String {
    notes
        .map(|n| format!("{n:?}"))
        .collect::<Vec<_>>()
        .join(", ")
}

pub(crate) fn to_upper_hex<'a>(notes: impl Iterator<Item = &'a Note>) -> String {
    notes
        .map(|n| format!("{n:X}"))
        .collect::<Vec<_>>()
        .join(", ")
}

pub(crate) fn to_lower_hex<'a>(notes: impl Iterator<Item = &'a Note>) -> String {
    notes
        .map(|n| format!("{n:x}"))
        .collect::<Vec<_>>()
        .join(", ")
}
