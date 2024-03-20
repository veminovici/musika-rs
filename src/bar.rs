use crate::{chords::Chords, Note};
use std::fmt::{Display, LowerHex, UpperHex};

pub enum BarElement {
    Silence(u8),
    Chord(Chords, u8),
    Note(Note, u8),
}

impl Display for BarElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BarElement::Silence(_) => write!(f, "_"),
            BarElement::Chord(chord, _) => write!(f, "{chord}"),
            BarElement::Note(note, _) => write!(f, "{note}"),
        }
    }
}

impl UpperHex for BarElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BarElement::Silence(_) => write!(f, "_"),
            BarElement::Chord(chord, _) => write!(f, "{chord:X}"),
            BarElement::Note(note, _) => write!(f, "{note:X}"),
        }
    }
}

impl LowerHex for BarElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BarElement::Silence(_) => write!(f, "_"),
            BarElement::Chord(chord, _) => write!(f, "{chord:x}"),
            BarElement::Note(note, _) => write!(f, "{note:x}"),
        }
    }
}

pub struct Bar(Vec<BarElement>);

impl Bar {
    const SEPARATOR: &'static str = " ";

    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn with_chord(self, chord: Chords, fraction: u8) -> Self {
        let mut items = self.0;
        items.push(BarElement::Chord(chord, fraction));
        Self(items)
    }

    pub fn with_note(self, note: Note, fraction: u8) -> Self {
        let mut items = self.0;
        items.push(BarElement::Note(note, fraction));
        Self(items)
    }

    pub fn with_silence(self, fraction: u8) -> Self {
        let mut items = self.0;
        items.push(BarElement::Silence(fraction));
        Self(items)
    }
}

impl Default for Bar {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Bar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .0
            .iter()
            .map(|e| format!("{e}"))
            .collect::<Vec<String>>()
            .join(Self::SEPARATOR);
        write!(f, "{s}")
    }
}

impl UpperHex for Bar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .0
            .iter()
            .map(|e| format!("{e:X}"))
            .collect::<Vec<String>>()
            .join(Self::SEPARATOR);
        write!(f, "{s}")
    }
}

impl LowerHex for Bar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .0
            .iter()
            .map(|e| format!("{e:x}"))
            .collect::<Vec<String>>()
            .join(Self::SEPARATOR);
        write!(f, "{s}")
    }
}

// pub fn show(bars: impl Iterator<Item = Bar>) -> String {
//     let s = bars
//         .map(|b| b.to_string())
//         .collect::<Vec<String>>()
//         .join(" | ");
//     format!("| {s} |")
// }

#[cfg(test)]
mod tests {
    use crate::{C, G};

    use super::*;

    #[test]
    fn display() {
        let bar = Bar::new().with_chord(C.maj(), 2).with_chord(G.maj(), 2);
        assert_eq!(bar.to_string(), "C G");
    }

    // #[test]
    // fn displa_bars() {
    //     let bar1 = Bar::new().with_chord(C.maj(), 2).with_chord(G.maj(), 2);
    //     let bar2 = Bar::new().with_chord(C.maj(), 2).with_chord(G.maj(), 2);
    //     let bars = [bar1, bar2];

    //     let s = show(bars.into_iter());
    //     assert_eq!(s, "| C - G | C - G |")
    // }
}
