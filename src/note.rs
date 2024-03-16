use crate::{Octave, Tone, OCTAVE_DEFAULT};
use std::{
    fmt::{Display, Octal},
    ops::Add,
};

const OCTAVE_NOTE_COUNT: u8 = 12;

pub const C: Note = Note {
    octave: OCTAVE_DEFAULT,
    note: 0,
};
pub const C_SHARP: Note = Note {
    octave: OCTAVE_DEFAULT,
    note: 1,
};
pub const D: Note = Note {
    octave: OCTAVE_DEFAULT,
    note: 2,
};
pub const D_SHARP: Note = Note {
    octave: OCTAVE_DEFAULT,
    note: 3,
};
pub const E: Note = Note {
    octave: OCTAVE_DEFAULT,
    note: 4,
};
pub const F: Note = Note {
    octave: OCTAVE_DEFAULT,
    note: 5,
};
pub const F_SHARP: Note = Note {
    octave: OCTAVE_DEFAULT,
    note: 6,
};
pub const G: Note = Note {
    octave: OCTAVE_DEFAULT,
    note: 7,
};
pub const G_SHARP: Note = Note {
    octave: OCTAVE_DEFAULT,
    note: 8,
};
pub const A: Note = Note {
    octave: OCTAVE_DEFAULT,
    note: 9,
};
pub const A_SHARP: Note = Note {
    octave: OCTAVE_DEFAULT,
    note: 10,
};
pub const B: Note = Note {
    octave: OCTAVE_DEFAULT,
    note: 11,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Note {
    octave: Octave,
    note: u8,
}

impl Note {
    pub fn octave(&self) -> &Octave {
        &self.octave
    }

    pub fn note(&self) -> u8 {
        self.note
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.note {
            0 => write!(f, "C"),
            1 => write!(f, "C#"),
            2 => write!(f, "D"),
            3 => write!(f, "D#"),
            4 => write!(f, "E"),
            5 => write!(f, "F"),
            6 => write!(f, "F#"),
            7 => write!(f, "G"),
            8 => write!(f, "G#"),
            9 => write!(f, "A"),
            10 => write!(f, "A#"),
            11 => write!(f, "B"),
            n => panic!("We should not have such note [{n}]"),
        }
    }
}

impl Octal for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.note {
            0 => write!(f, "{}:C", self.octave),
            1 => write!(f, "{}:C#", self.octave),
            2 => write!(f, "{}:D", self.octave),
            3 => write!(f, "{}:D#", self.octave),
            4 => write!(f, "{}:E", self.octave),
            5 => write!(f, "{}:F", self.octave),
            6 => write!(f, "{}:F#", self.octave),
            7 => write!(f, "{}:G", self.octave),
            8 => write!(f, "{}:G#", self.octave),
            9 => write!(f, "{}:A", self.octave),
            10 => write!(f, "{}:A#", self.octave),
            11 => write!(f, "{}:B", self.octave),
            n => panic!("We should not have such note [{n}]"),
        }
    }
}

impl From<u8> for Note {
    fn from(value: u8) -> Self {
        let octave = value / OCTAVE_NOTE_COUNT;
        let note = value % OCTAVE_NOTE_COUNT;
        Note {
            octave: Octave::from(octave),
            note,
        }
    }
}

impl From<Note> for u8 {
    fn from(note: Note) -> Self {
        let octave = u8::from(note.octave);
        let note = note.note;
        octave * OCTAVE_NOTE_COUNT + note
    }
}

impl Add<Tone> for Note {
    type Output = Note;

    fn add(self, tone: Tone) -> Self::Output {
        let note = u8::from(self) + u8::from(tone);
        Note::from(note)
    }
}

pub struct Notes<const N: usize> {
    root: Note,
    notes: [Note; N],
}

impl<const N: usize> Notes<N> {
    pub fn new(root: &Note, steps: [Tone; N]) -> Self {
        let notes = NoteStepperIterator::new(root, steps.into_iter())
            .skip(1)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Self { root: *root, notes }
    }

    pub fn len(&self) -> usize {
        N + 1
    }

    pub fn root(&self) -> &Note {
        &self.root
    }

    pub fn index(&self, idx: usize) -> Option<&Note> {
        match idx {
            0 => Some(&self.root),
            n if n <= N => Some(&self.notes[idx - 1]),
            _ => None,
        }
    }
}

impl<const N: usize> IntoIterator for Notes<N> {
    type Item = Note;

    type IntoIter = NoteCollectionIterator<N>;

    fn into_iter(self) -> Self::IntoIter {
        NoteCollectionIterator::new(self)
    }
}
pub struct NoteCollectionIterator<const N: usize> {
    notes: Notes<N>,
    idx: usize,
}

impl<const N: usize> NoteCollectionIterator<N> {
    pub(crate) fn new(notes: Notes<N>) -> Self {
        Self { notes, idx: 0 }
    }
}

impl<const N: usize> Iterator for NoteCollectionIterator<N> {
    type Item = Note;

    fn next(&mut self) -> Option<Self::Item> {
        let note = self.notes.index(self.idx).map(|n| *n);
        self.idx += 1;
        note
    }
}

pub(crate) struct NoteStepperIterator<S> {
    cur_note: Note,
    started: bool,
    steps: S,
}

impl<S> NoteStepperIterator<S> {
    pub(crate) fn new(tonic: &Note, steps: S) -> Self {
        Self {
            cur_note: *tonic,
            started: false,
            steps,
        }
    }
}

impl<S> Iterator for NoteStepperIterator<S>
where
    S: Iterator<Item = Tone>,
{
    type Item = Note;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            self.started = true;
            return Some(self.cur_note);
        }

        self.steps
            .next()
            .map(|s| self.cur_note + s)
            .inspect(|n| self.cur_note = *n)
    }
}

#[cfg(test)]
mod tests {
    use crate::{C5, H, W};

    use super::*;

    #[test]
    fn display() {
        assert_eq!(format!("{C}"), "C");
        assert_eq!(format!("{C_SHARP}"), "C#");
        assert_eq!(format!("{D}"), "D");
        assert_eq!(format!("{D_SHARP}"), "D#");
        assert_eq!(format!("{E}"), "E");
        assert_eq!(format!("{F}"), "F");
        assert_eq!(format!("{F_SHARP}"), "F#");
        assert_eq!(format!("{G}"), "G");
        assert_eq!(format!("{G_SHARP}"), "G#");
        assert_eq!(format!("{A}"), "A");
        assert_eq!(format!("{A_SHARP}"), "A#");
        // assert_eq!(format!("{B}"), "B");
    }

    #[test]
    fn octal() {
        assert_eq!(format!("{C:o}"), "C4:C");
        assert_eq!(format!("{C_SHARP:o}"), "C4:C#");
        assert_eq!(format!("{D:o}"), "C4:D");
        assert_eq!(format!("{D_SHARP:o}"), "C4:D#");
        assert_eq!(format!("{E:o}"), "C4:E");
        assert_eq!(format!("{F:o}"), "C4:F");
        assert_eq!(format!("{F_SHARP:o}"), "C4:F#");
        assert_eq!(format!("{G:o}"), "C4:G");
        assert_eq!(format!("{G_SHARP:o}"), "C4:G#");
        assert_eq!(format!("{A:o}"), "C4:A");
        assert_eq!(format!("{A_SHARP:o}"), "C4:A#");
        assert_eq!(format!("{B:o}"), "C4:B");
    }

    #[test]
    fn u8_to_note() {
        assert_eq!(Note::from(4 * OCTAVE_NOTE_COUNT + 0), C);
        assert_eq!(Note::from(4 * OCTAVE_NOTE_COUNT + 1), C_SHARP);
        assert_eq!(Note::from(4 * OCTAVE_NOTE_COUNT + 2), D);
        assert_eq!(Note::from(4 * OCTAVE_NOTE_COUNT + 3), D_SHARP);
        assert_eq!(Note::from(4 * OCTAVE_NOTE_COUNT + 4), E);
        assert_eq!(Note::from(4 * OCTAVE_NOTE_COUNT + 5), F);
        assert_eq!(Note::from(4 * OCTAVE_NOTE_COUNT + 6), F_SHARP);
        assert_eq!(Note::from(4 * OCTAVE_NOTE_COUNT + 7), G);
        assert_eq!(Note::from(4 * OCTAVE_NOTE_COUNT + 8), G_SHARP);
        assert_eq!(Note::from(4 * OCTAVE_NOTE_COUNT + 9), A);
        assert_eq!(Note::from(4 * OCTAVE_NOTE_COUNT + 10), A_SHARP);
        assert_eq!(Note::from(4 * OCTAVE_NOTE_COUNT + 11), B);
    }

    #[test]
    fn note_to_u8() {
        assert_eq!(u8::from(C), 4 * OCTAVE_NOTE_COUNT + 0);
        assert_eq!(u8::from(C_SHARP), 4 * OCTAVE_NOTE_COUNT + 1);
        assert_eq!(u8::from(D), 4 * OCTAVE_NOTE_COUNT + 2);
        assert_eq!(u8::from(D_SHARP), 4 * OCTAVE_NOTE_COUNT + 3);
        assert_eq!(u8::from(E), 4 * OCTAVE_NOTE_COUNT + 4);
        assert_eq!(u8::from(F), 4 * OCTAVE_NOTE_COUNT + 5);
        assert_eq!(u8::from(F_SHARP), 4 * OCTAVE_NOTE_COUNT + 6);
        assert_eq!(u8::from(G), 4 * OCTAVE_NOTE_COUNT + 7);
        assert_eq!(u8::from(G_SHARP), 4 * OCTAVE_NOTE_COUNT + 8);
        assert_eq!(u8::from(A), 4 * OCTAVE_NOTE_COUNT + 9);
        assert_eq!(u8::from(A_SHARP), 4 * OCTAVE_NOTE_COUNT + 10);
        assert_eq!(u8::from(B), 4 * OCTAVE_NOTE_COUNT + 11);
    }

    #[test]
    fn add() {
        assert_eq!(C + H, C_SHARP);
        assert_eq!(C + W, D);
        assert_eq!(C + Tone::from(3), D_SHARP);

        let note = B + H;
        assert_eq!(note.octave, C5);
        assert_eq!(note.note, 0);

        let note = B + W;
        assert_eq!(note.octave, C5);
        assert_eq!(note.note, 1);
    }

    #[test]
    fn stepper() {
        let tonic = C;
        let steps = [H, H];
        let mut iter = NoteStepperIterator::new(&tonic, steps.into_iter());

        assert_eq!(iter.next(), Some(C));
        assert_eq!(iter.next(), Some(C_SHARP));
        assert_eq!(iter.next(), Some(D));
        assert!(iter.next().is_none());
    }
}
