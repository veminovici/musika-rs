use std::{
    fmt::{Debug, Display, LowerHex, UpperHex},
    ops::{Add, Sub},
};

use crate::{
    chords::{self, Chords},
    Interval, PERFECT_5TH,
};

use super::{Tone, OCTAVE};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Note(i8);

impl Note {
    const TRANSLATE: i8 = 3;
    const OCTAVE_SIZE: i8 = OCTAVE.inner() as i8;

    /// Returns the base note (the one in the C4 octave)
    pub fn base(&self) -> Self {
        let mut note = self.0 + Self::TRANSLATE;
        if note < 0 {
            let p = note / Self::OCTAVE_SIZE;
            note += (p.abs() + 1) * Self::OCTAVE_SIZE;
        }

        note %= Self::OCTAVE_SIZE;
        note -= Self::TRANSLATE;

        let b: Self = note.into();
        debug_assert_eq!(
            b.octave(),
            4,
            "The base is not C4: self={} note={}",
            self.0,
            note
        );
        b
    }

    /// Returns the octave for the given note (eg. C4)
    pub fn octave(&self) -> i8 {
        let octave = (self.0 + Self::TRANSLATE) / Self::OCTAVE_SIZE;
        let rest = (self.0 + Self::TRANSLATE) % Self::OCTAVE_SIZE;
        if rest >= 0 {
            octave + Self::TRANSLATE + 1
        } else {
            octave + Self::TRANSLATE
        }
    }

    pub fn perfect_fifth(&self) -> Self {
        self + PERFECT_5TH
    }

    //
    // Functions which build chords
    //

    //
    // Diminished chords
    //

    /// Builds a **dim** chord with the root in the current note.
    pub fn dim(self) -> chords::Chords {
        chords::dim(self)
    }

    /// Builds a **dim7** chord with the root in the current note.
    ///
    /// # Example
    /// ```
    /// use musika_rs::*;
    ///
    /// let chord = C.dim7();
    /// println!("{:X}", chord);
    /// ```
    pub fn dim7(self) -> chords::Chords {
        chords::dim7(self)
    }

    /// Return all the diminished chords for the current note.
    pub fn dimished_chords(self) -> impl Iterator<Item = chords::Chords> {
        chords::diminished_chords(self)
    }

    //
    // Dominant chords
    //

    /// Builds a dominant7 choard with the root in the current note.
    ///
    /// # Example
    /// ```
    /// use musika_rs::*;
    ///
    /// let chord = C.dom7();
    /// println!("{:X}", chord);
    /// ```
    pub fn dom7(self) -> chords::Chords {
        chords::dom7(self)
    }

    pub fn dom7b5(self) -> chords::Chords {
        chords::dom7b5(self)
    }

    pub fn dom7s5(self) -> chords::Chords {
        chords::dom7s5(self)
    }

    pub fn dom9(self) -> chords::Chords {
        chords::dom9(self)
    }

    pub fn dom11(self) -> chords::Chords {
        chords::dom11(self)
    }

    pub fn dom13(self) -> chords::Chords {
        chords::dom13(self)
    }

    pub fn dom13b9b13(self) -> chords::Chords {
        chords::dom13b9b13(self)
    }

    pub fn dominant_chords(self) -> impl Iterator<Item = Chords> {
        chords::dominant_chords(self)
    }

    /// Builds a major chord with the root in the current note.
    ///
    /// # Example
    /// ```
    /// use musika_rs::*;
    ///
    /// let chord = C.maj();
    /// println!("{:X}", chord);
    /// ```
    pub fn maj(self) -> chords::Chords {
        chords::maj(self)
    }

    /// Builds a major7 chord with the root in the current note.
    ///
    /// # Example
    /// ```
    /// use musika_rs::*;
    ///
    /// let chord = C.maj7();
    /// println!("{:X}", chord);
    /// ```
    pub fn maj7(self) -> chords::Chords {
        chords::maj7(self)
    }

    pub fn maj9(self) -> chords::Chords {
        chords::maj9(self)
    }

    pub fn maj11(self) -> chords::Chords {
        chords::maj11(self)
    }

    pub fn maj13(self) -> chords::Chords {
        chords::maj13(self)
    }

    pub fn major_chords(self) -> impl Iterator<Item = Chords> {
        chords::major_chords(self)
    }

    /// Builds a minor chord with the root in the current note.
    ///
    /// # Example
    /// ```
    /// use musika_rs::*;
    ///
    /// let chord = C.min();
    /// println!("{:X}", chord);
    /// ```
    pub fn min(self) -> chords::Chords {
        chords::min(self)
    }

    /// Builds a minor7 chord with the root in the current note.
    ///
    /// # Example
    /// ```
    /// use musika_rs::*;
    ///
    /// let chord = C.min7();
    /// println!("{:X}", chord);
    /// ```
    pub fn min7(self) -> chords::Chords {
        chords::min7(self)
    }

    pub fn min7b5(self) -> chords::Chords {
        chords::min7b5(self)
    }

    pub fn min9(self) -> chords::Chords {
        chords::min9(self)
    }

    pub fn min11(self) -> chords::Chords {
        chords::min11(self)
    }

    pub fn min13(self) -> chords::Chords {
        chords::min13(self)
    }

    pub fn minor_chords(self) -> impl Iterator<Item = Chords> {
        chords::minor_chords(self)
    }

    pub fn all_chords(self) -> impl Iterator<Item = Chords> {
        chords::Chords::all_chords(self)
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:X}")
    }
}

impl Debug for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "C{}:{:X}:{}", self.octave(), self.base(), self.0)
    }
}

impl UpperHex for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base = self.base();
        match base.0 {
            -3 => write!(f, "A"),
            -2 => write!(f, "A#"),
            -1 => write!(f, "B"),
            0 => write!(f, "C"),
            1 => write!(f, "C#"),
            2 => write!(f, "D"),
            3 => write!(f, "D#"),
            4 => write!(f, "E"),
            5 => write!(f, "F"),
            6 => write!(f, "F#"),
            7 => write!(f, "G"),
            8 => write!(f, "G#"),
            n => panic!("We should not be here [{n}]"),
        }
    }
}

impl LowerHex for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base = self.base();
        match base.0 {
            -3 => write!(f, "A"),
            -2 => write!(f, "Bb"),
            -1 => write!(f, "B"),
            0 => write!(f, "C"),
            1 => write!(f, "Db"),
            2 => write!(f, "D"),
            3 => write!(f, "Eb"),
            4 => write!(f, "E"),
            5 => write!(f, "F"),
            6 => write!(f, "Gb"),
            7 => write!(f, "G"),
            8 => write!(f, "Ab"),
            n => panic!("We should not be here [{n}]"),
        }
    }
}

impl AsRef<i8> for Note {
    fn as_ref(&self) -> &i8 {
        &self.0
    }
}

impl From<i8> for Note {
    fn from(value: i8) -> Self {
        Self(value)
    }
}

impl From<Note> for i8 {
    fn from(note: Note) -> Self {
        note.0
    }
}

impl Add<Tone> for Note {
    type Output = Self;

    fn add(self, tone: Tone) -> Self::Output {
        let note = self.0 + u8::from(tone) as i8;
        note.into()
    }
}

impl Add<Interval> for Note {
    type Output = Self;

    fn add(self, interval: Interval) -> Self::Output {
        let note = self.0 + u8::from(interval) as i8;
        note.into()
    }
}

impl Add<Interval> for &Note {
    type Output = Note;

    fn add(self, interval: Interval) -> Self::Output {
        let note = self.0 + u8::from(interval) as i8;
        note.into()
    }
}

impl Sub<Tone> for Note {
    type Output = Self;

    fn sub(self, tone: Tone) -> Self::Output {
        let note = self.0 - u8::from(tone) as i8;
        note.into()
    }
}

impl Sub<Note> for Note {
    type Output = Tone;

    fn sub(self, other: Note) -> Self::Output {
        let tone = (self.0 - other.0).unsigned_abs();
        tone.into()
    }
}

pub const A: Note = Note(-Note::TRANSLATE);
pub const A_SHARP: Note = Note(-Note::TRANSLATE + 1);
pub const B: Note = Note(-Note::TRANSLATE + 2);
pub const C: Note = Note(0);
pub const C_SHARP: Note = Note(1);
pub const D: Note = Note(2);
pub const D_SHARP: Note = Note(3);
pub const E: Note = Note(4);
pub const F: Note = Note(5);
pub const F_SHARP: Note = Note(6);
pub const G: Note = Note(7);
pub const G_SHARP: Note = Note(8);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{SEMI_TONE, TONE};

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
        assert_eq!(format!("{B}"), "B");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{A:?}"), "C4:A:-3");
        assert_eq!(format!("{A_SHARP:?}"), "C4:A#:-2");
        assert_eq!(format!("{B:?}"), "C4:B:-1");
        assert_eq!(format!("{C:?}"), "C4:C:0");
        assert_eq!(format!("{C_SHARP:?}"), "C4:C#:1");
        assert_eq!(format!("{D:?}"), "C4:D:2");
        assert_eq!(format!("{D_SHARP:?}"), "C4:D#:3");
        assert_eq!(format!("{E:?}"), "C4:E:4");
        assert_eq!(format!("{F:?}"), "C4:F:5");
        assert_eq!(format!("{F_SHARP:?}"), "C4:F#:6");
        assert_eq!(format!("{G:?}"), "C4:G:7");
        assert_eq!(format!("{G_SHARP:?}"), "C4:G#:8");
    }

    #[test]
    fn upper_hex() {
        assert_eq!(format!("{C:X}"), "C");
        assert_eq!(format!("{C_SHARP:X}"), "C#");
        assert_eq!(format!("{D:X}"), "D");
        assert_eq!(format!("{D_SHARP:X}"), "D#");
        assert_eq!(format!("{E:X}"), "E");
        assert_eq!(format!("{F:X}"), "F");
        assert_eq!(format!("{F_SHARP:X}"), "F#");
        assert_eq!(format!("{G:X}"), "G");
        assert_eq!(format!("{G_SHARP:X}"), "G#");
        assert_eq!(format!("{A:X}"), "A");
        assert_eq!(format!("{A_SHARP:X}"), "A#");
        assert_eq!(format!("{B:X}"), "B");
    }

    #[test]
    fn lower_hex() {
        assert_eq!(format!("{C:x}"), "C");
        assert_eq!(format!("{C_SHARP:x}"), "Db");
        assert_eq!(format!("{D:x}"), "D");
        assert_eq!(format!("{D_SHARP:x}"), "Eb");
        assert_eq!(format!("{E:x}"), "E");
        assert_eq!(format!("{F:x}"), "F");
        assert_eq!(format!("{F_SHARP:x}"), "Gb");
        assert_eq!(format!("{G:x}"), "G");
        assert_eq!(format!("{G_SHARP:x}"), "Ab");
        assert_eq!(format!("{A:x}"), "A");
        assert_eq!(format!("{A_SHARP:x}"), "Bb");
        assert_eq!(format!("{B:x}"), "B");
    }

    #[test]
    fn i8_to_note() {
        assert_eq!(Note::from(-3), A);
        assert_eq!(Note::from(-2), A_SHARP);
        assert_eq!(Note::from(-1), B);
        assert_eq!(Note::from(0), C);
        assert_eq!(Note::from(1), C_SHARP);
        assert_eq!(Note::from(2), D);
        assert_eq!(Note::from(3), D_SHARP);
        assert_eq!(Note::from(4), E);
        assert_eq!(Note::from(5), F);
        assert_eq!(Note::from(6), F_SHARP);
        assert_eq!(Note::from(7), G);
        assert_eq!(Note::from(8), G_SHARP);
    }

    #[test]
    fn note_to_i8() {
        assert_eq!(i8::from(A), -3);
        assert_eq!(i8::from(A_SHARP), -2);
        assert_eq!(i8::from(B), -1);
        assert_eq!(i8::from(C), 0);
        assert_eq!(i8::from(C_SHARP), 1);
        assert_eq!(i8::from(D), 2);
        assert_eq!(i8::from(D_SHARP), 3);
        assert_eq!(i8::from(E), 4);
        assert_eq!(i8::from(F), 5);
        assert_eq!(i8::from(F_SHARP), 6);
        assert_eq!(i8::from(G), 7);
        assert_eq!(i8::from(G_SHARP), 8);
    }

    #[test]
    fn octave() {
        assert_eq!(A.octave(), 4);
        assert_eq!(A_SHARP.octave(), 4);
        assert_eq!(B.octave(), 4);
        assert_eq!(C.octave(), 4);
        assert_eq!(C_SHARP.octave(), 4);
        assert_eq!(D.octave(), 4);
        assert_eq!(D_SHARP.octave(), 4);
        assert_eq!(E.octave(), 4);
        assert_eq!(F.octave(), 4);
        assert_eq!(F_SHARP.octave(), 4);
        assert_eq!(G.octave(), 4);
        assert_eq!(G_SHARP.octave(), 4);

        let note = Note::from(9);
        assert_eq!(note.base(), A);
        assert_eq!(note.octave(), 5);

        let note = Note::from(9 + 12);
        assert_eq!(note.base(), A);
        assert_eq!(note.octave(), 6);

        let note = Note::from(-4);
        assert_eq!(note.base(), G_SHARP);
        assert_eq!(note.octave(), 3);

        let note = note - OCTAVE;
        assert_eq!(note.base(), G_SHARP);
        assert_eq!(note.octave(), 2);
    }

    #[test]
    fn add() {
        assert_eq!(C + SEMI_TONE, C_SHARP);
        assert_eq!(C + TONE, D);
        assert_eq!(C + Tone::from(3), D_SHARP);

        let note = G_SHARP + SEMI_TONE;
        assert_eq!(note.octave(), 5);
        assert_eq!(note.base(), A);

        let note = G_SHARP + TONE;
        assert_eq!(note.octave(), 5);
        assert_eq!(note.base(), A_SHARP);
    }

    #[test]
    fn sub_tone() {
        assert_eq!(G - SEMI_TONE, F_SHARP);
        assert_eq!(G - TONE, F);

        let note = C - SEMI_TONE;
        assert_eq!(note.octave(), 4);
        assert_eq!(note.base(), B);

        let note = C - TONE;
        assert_eq!(note.octave(), 4);
        assert_eq!(note.base(), A_SHARP);

        let note = A - SEMI_TONE;
        assert_eq!(note.octave(), 3);
        assert_eq!(note.base(), G_SHARP);
    }

    #[test]
    fn sub_note() {
        assert_eq!(C - B, SEMI_TONE);
    }

    #[test]
    fn perfect_fifth() {
        assert_eq!(D.perfect_fifth().base(), A.base());
        assert_eq!(A.perfect_fifth().base(), E.base());
        assert_eq!(E.perfect_fifth().base(), B.base());
    }
}

pub(crate) struct NoteStepperIterator<S> {
    cur_note: Note,
    started: bool,
    steps: S,
}

impl<S, T> NoteStepperIterator<S>
where
    S: Iterator<Item = T>,
    Tone: From<T>,
{
    pub(crate) fn new(root: Note, steps: S) -> Self {
        Self {
            cur_note: root,
            started: false,
            steps,
        }
    }
}

impl<S, T> Iterator for NoteStepperIterator<S>
where
    S: Iterator<Item = T>,
    Tone: From<T>,
{
    type Item = Note;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            self.started = true;
            return Some(self.cur_note);
        }

        self.steps
            .next()
            .map(|s| self.cur_note + Tone::from(s))
            .inspect(|n| self.cur_note = *n)
    }
}
