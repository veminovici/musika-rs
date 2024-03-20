use crate::{Note, NoteStepperIterator, Tone};
use std::fmt::{Display, LowerHex, UpperHex};

mod major;
mod minor;

pub use major::*;
pub use minor::*;

pub trait Scale {
    fn name(&self) -> &'static str;
    fn tonic(&self) -> Note;
    fn notes(&self) -> impl Iterator<Item = &Note>;
    fn as_steps(&self) -> impl Iterator<Item = Tone>;
}

pub enum Scales {
    Major(&'static str, Vec<Note>),
    Minor(&'static str, Vec<Note>),
}

impl Scales {
    fn major<N>(name: &'static str, notes: N) -> Self
    where
        N: Iterator<Item = Note>,
    {
        Self::Major(name, notes.map(|n| n.base()).collect())
    }

    fn major_with_steps<S, T>(name: &'static str, root: Note, steps: S) -> Self
    where
        S: Iterator<Item = T>,
        Tone: From<T>,
    {
        let notes = NoteStepperIterator::new(root, steps.into_iter());
        Self::major(name, notes)
    }

    fn minor<N>(name: &'static str, notes: N) -> Self
    where
        N: Iterator<Item = Note>,
    {
        Self::Minor(name, notes.map(|n| n.base()).collect())
    }

    fn minor_with_steps<S, T>(name: &'static str, root: Note, steps: S) -> Self
    where
        S: Iterator<Item = T>,
        Tone: From<T>,
    {
        let notes = NoteStepperIterator::new(root, steps.into_iter());
        Self::minor(name, notes)
    }

    fn inner_notes(&self) -> &Vec<Note> {
        match self {
            Scales::Major(_, notes) => notes,
            Scales::Minor(_, notes) => notes,
        }
    }

    fn inner_name(&self) -> &'static str {
        match self {
            Scales::Major(name, _) => name,
            Scales::Minor(name, _) => name,
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

impl Scale for Scales {
    fn name(&self) -> &'static str {
        self.inner_name()
    }

    fn tonic(&self) -> Note {
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

impl Display for Scales {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.tonic(), self.inner_name())
    }
}

impl UpperHex for Scales {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let notes = self.notes_upper_hex();
        write!(f, "{self} [{notes}]")
    }
}

impl LowerHex for Scales {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let notes = self.notes_lower_hex();
        write!(f, "{self} [{notes}]")
    }
}
