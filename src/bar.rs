use std::fmt::Display;

use crate::{chords::Chord, Note};

pub enum BarElement<C>
where
    C: Chord<IntoIter = <Vec<Note> as IntoIterator>::IntoIter>,
{
    Silence(u8),
    Chord(C, u8),
}

impl<C> Display for BarElement<C>
where
    C: Display + Chord<IntoIter = <Vec<Note> as IntoIterator>::IntoIter>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BarElement::Silence(_) => write!(f, "_"),
            BarElement::Chord(chord, _) => write!(f, "{}", chord),
        }
    }
}

pub struct Bar<C>(Vec<BarElement<C>>)
where
    C: Chord<IntoIter = <Vec<Note> as IntoIterator>::IntoIter>;

impl<C> Bar<C>
where
    C: Chord<IntoIter = <Vec<Note> as IntoIterator>::IntoIter>,
{
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn with_chord(self, chord: C, fraction: u8) -> Self {
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

impl<C> Default for Bar<C>
where
    C: Chord<IntoIter = <Vec<Note> as IntoIterator>::IntoIter>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<C> Display for Bar<C>
where
    C: Display + Chord<IntoIter = <Vec<Note> as IntoIterator>::IntoIter>,
{
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

pub fn show<C>(bars: impl Iterator<Item = Bar<C>>) -> String
where
    C: Display + Chord<IntoIter = <Vec<Note> as IntoIterator>::IntoIter>,
{
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
        let bar = Bar::new().with_chord(C.major(), 2).with_chord(G.major(), 2);
        assert_eq!(bar.to_string(), "C - G");
    }

    #[test]
    fn displa_bars() {
        let bar1 = Bar::new().with_chord(C.major(), 2).with_chord(G.major(), 2);
        let bar2 = Bar::new().with_chord(C.major(), 2).with_chord(G.major(), 2);
        let bars = [bar1, bar2];

        let s = show(bars.into_iter());
        assert_eq!(s, "| C - G | C - G |")
    }
}
