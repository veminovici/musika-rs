use crate::{Note, NoteStepperIterator, Tone};
use std::fmt::{Display, LowerHex, UpperHex};

mod diminished;
mod dominant;
mod major;
mod minor;

pub use diminished::*;
pub use dominant::*;
pub use major::*;
pub use minor::*;

pub trait XChord {
    fn root(&self) -> Note;
    fn notes(&self) -> impl Iterator<Item = &Note>;
    fn as_steps(&self) -> impl Iterator<Item = Tone>;
}

pub enum XChords {
    Major(&'static str, Vec<Note>),
    Minor(&'static str, Vec<Note>),
    Dominant(&'static str, Vec<Note>),
    Diminished(&'static str, Vec<Note>),
}

impl XChords {
    fn major(name: &'static str, notes: impl Iterator<Item = Note>) -> Self {
        Self::Major(name, notes.collect())
    }

    pub fn major_with_steps<T>(
        name: &'static str,
        root: Note,
        steps: impl Iterator<Item = T>,
    ) -> Self
    where
        Tone: From<T>,
    {
        let notes = NoteStepperIterator::new(root, steps.into_iter());
        Self::major(name, notes)
    }

    fn minor(name: &'static str, notes: impl Iterator<Item = Note>) -> Self {
        Self::Minor(name, notes.collect())
    }

    pub fn minor_with_steps<T>(
        name: &'static str,
        root: Note,
        steps: impl Iterator<Item = T>,
    ) -> Self
    where
        Tone: From<T>,
    {
        let notes = NoteStepperIterator::new(root, steps.into_iter());
        Self::minor(name, notes)
    }

    fn dominant(name: &'static str, notes: impl Iterator<Item = Note>) -> Self {
        Self::Dominant(name, notes.collect())
    }

    pub fn dominant_with_steps<T>(
        name: &'static str,
        root: Note,
        steps: impl Iterator<Item = T>,
    ) -> Self
    where
        Tone: From<T>,
    {
        let notes = NoteStepperIterator::new(root, steps.into_iter());
        Self::dominant(name, notes)
    }

    fn diminished(name: &'static str, notes: impl Iterator<Item = Note>) -> Self {
        Self::Diminished(name, notes.collect())
    }

    pub fn diminished_with_steps<T>(
        name: &'static str,
        root: Note,
        steps: impl Iterator<Item = T>,
    ) -> Self
    where
        Tone: From<T>,
    {
        let notes = NoteStepperIterator::new(root, steps.into_iter());
        Self::diminished(name, notes)
    }

    fn inner_notes(&self) -> &Vec<Note> {
        match self {
            XChords::Major(_, notes) => notes,
            XChords::Minor(_, notes) => notes,
            XChords::Dominant(_, notes) => notes,
            XChords::Diminished(_, notes) => notes,
        }
    }

    fn inner_name(&self) -> &'static str {
        match self {
            XChords::Major(name, _) => name,
            XChords::Minor(name, _) => name,
            XChords::Dominant(name, _) => name,
            XChords::Diminished(name, _) => name,
        }
    }

    const SEPARATOR: &'static str = ", ";

    fn notes_upper_hex(&self) -> String {
        self.inner_notes()
            .iter()
            .map(|n| format!("{n:X}"))
            .collect::<Vec<_>>()
            .join(Self::SEPARATOR)
    }

    fn notes_lower_hex(&self) -> String {
        self.inner_notes()
            .iter()
            .map(|n| format!("{n:x}"))
            .collect::<Vec<_>>()
            .join(Self::SEPARATOR)
    }
}

impl XChord for XChords {
    fn root(&self) -> Note {
        self.inner_notes()[0]
    }

    fn notes(&self) -> impl Iterator<Item = &Note> {
        self.inner_notes().iter()
    }

    fn as_steps(&self) -> impl Iterator<Item = Tone> {
        self.inner_notes()
            .as_slice()
            .windows(2)
            .map(|notes| notes[0] - notes[1])
    }
}

impl Display for XChords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.root(), self.inner_name())
    }
}

impl UpperHex for XChords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let notes = self.notes_upper_hex();
        write!(f, "{self} [{notes}]")
    }
}

impl LowerHex for XChords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let notes = self.notes_lower_hex();
        write!(f, "{self} [{notes}]")
    }
}
