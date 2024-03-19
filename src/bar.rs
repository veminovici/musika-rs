use crate::chords::Chords;
use std::fmt::Display;

pub enum BarElement {
    Silence(u8),
    Chord(Chords, u8),
}

impl Display for BarElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BarElement::Silence(_) => write!(f, "_"),
            BarElement::Chord(chord, _) => write!(f, "{}", chord),
        }
    }
}

pub struct Bar(Vec<BarElement>);

impl Bar {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn with_chord(self, chord: Chords, fraction: u8) -> Self {
        let mut items = self.0;
        items.push(BarElement::Chord(chord, fraction));
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
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(" - ");
        write!(f, "{s}")
    }
}

pub fn show(bars: impl Iterator<Item = Bar>) -> String {
    let s = bars
        .map(|b| b.to_string())
        .collect::<Vec<String>>()
        .join(" | ");
    format!("| {s} |")
}

#[cfg(test)]
mod tests {
    use crate::{C, G};

    use super::*;

    #[test]
    fn display() {
        let bar = Bar::new().with_chord(C.maj(), 2).with_chord(G.maj(), 2);
        assert_eq!(bar.to_string(), "C - G");
    }

    #[test]
    fn displa_bars() {
        let bar1 = Bar::new().with_chord(C.maj(), 2).with_chord(G.maj(), 2);
        let bar2 = Bar::new().with_chord(C.maj(), 2).with_chord(G.maj(), 2);
        let bars = [bar1, bar2];

        let s = show(bars.into_iter());
        assert_eq!(s, "| C - G | C - G |")
    }
}
