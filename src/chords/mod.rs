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

pub trait Chord {
    fn root(&self) -> Note;
    fn notes(&self) -> impl Iterator<Item = &Note>;
    fn as_steps(&self) -> impl Iterator<Item = Tone>;
}

pub enum Chords {
    Major(&'static str, Vec<Note>),
    Minor(&'static str, Vec<Note>),
    Dominant(&'static str, Vec<Note>),
    Diminished(&'static str, Vec<Note>),
}

impl Chords {
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

    fn dominant<N>(name: &'static str, notes: N) -> Self
    where
        N: Iterator<Item = Note>,
    {
        Self::Dominant(name, notes.map(|n| n.base()).collect())
    }

    fn dominant_with_steps<S, T>(name: &'static str, root: Note, steps: S) -> Self
    where
        S: Iterator<Item = T>,
        Tone: From<T>,
    {
        let notes = NoteStepperIterator::new(root, steps.into_iter());
        Self::dominant(name, notes)
    }

    fn diminished<N>(name: &'static str, notes: N) -> Self
    where
        N: Iterator<Item = Note>,
    {
        Self::Diminished(name, notes.map(|n| n.base()).collect())
    }

    pub fn diminished_with_steps<S, T>(name: &'static str, root: Note, steps: S) -> Self
    where
        S: Iterator<Item = T>,
        Tone: From<T>,
    {
        let notes = NoteStepperIterator::new(root, steps.into_iter());
        Self::diminished(name, notes)
    }

    pub fn contains_notes<N>(&self, others: &mut N) -> bool
    where
        N: Iterator<Item = Note>,
    {
        let ns = self.inner_notes();
        others.into_iter().all(|note| ns.contains(&note))
    }

    pub fn find<P>(root: Note, predicate: P) -> impl Iterator<Item = Self>
    where
        P: FnMut(&Self) -> bool,
    {
        let mut chords = major_chords(root).collect::<Vec<_>>();
        chords.extend(minor_chords(root));
        chords.extend(dominant_chords(root));
        chords.extend(diminished_chords(root));

        chords.into_iter().filter(predicate)
    }

    pub fn all_chords(root: Note) -> impl Iterator<Item = Self> {
        Self::find(root, |_| true)
    }

    pub fn find_contain_notes<N>(root: Note, notes: N) -> impl Iterator<Item = Self>
    where
        N: Iterator<Item = Note>,
    {
        let notes = notes.collect::<Vec<_>>();

        Self::find(root, move |chord| {
            chord.contains_notes(&mut notes.clone().into_iter())
        })
    }

    fn inner_notes(&self) -> &Vec<Note> {
        match self {
            Chords::Major(_, notes) => notes,
            Chords::Minor(_, notes) => notes,
            Chords::Dominant(_, notes) => notes,
            Chords::Diminished(_, notes) => notes,
        }
    }

    fn inner_name(&self) -> &'static str {
        match self {
            Chords::Major(name, _) => name,
            Chords::Minor(name, _) => name,
            Chords::Dominant(name, _) => name,
            Chords::Diminished(name, _) => name,
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

impl Chord for Chords {
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

impl Display for Chords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.root(), self.inner_name())
    }
}

impl UpperHex for Chords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let notes = self.notes_upper_hex();
        write!(f, "{self} [{notes}]")
    }
}

impl LowerHex for Chords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let notes = self.notes_lower_hex();
        write!(f, "{self} [{notes}]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{C, E, G};

    #[test]
    fn find_notes() {
        let notes = [C, E, G];
        let res = Chords::find_contain_notes(C, notes.into_iter()).collect::<Vec<Chords>>();
        assert!(!res.is_empty());
    }
}
