use std::fmt::{Debug, Display};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Tone(u8);

pub const SEMI_TONE: Tone = Tone(1);
pub const TONE: Tone = Tone(2);
pub const OCTAVE: Tone = Tone(12);

impl Display for Tone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            1 => write!(f, "H"),
            2 => write!(f, "W"),
            12 => write!(f, "O"),
            n => write!(f, "T({n})"),
        }
    }
}

impl Debug for Tone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            1 => write!(f, "H(1)"),
            2 => write!(f, "W(2)"),
            12 => write!(f, "O(12)"),
            n => write!(f, "T({n})"),
        }
    }
}

impl AsRef<u8> for Tone {
    fn as_ref(&self) -> &u8 {
        &self.0
    }
}

impl From<u8> for Tone {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<Tone> for u8 {
    fn from(tone: Tone) -> Self {
        tone.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(format!("{SEMI_TONE}"), "H");
        assert_eq!(format!("{TONE}"), "W");
        assert_eq!(format!("{OCTAVE}"), "O");
        assert_eq!(format!("{}", Tone(3)), "T(3)");
    }

    #[test]
    fn u8_to_tone() {
        assert_eq!(Tone::from(1), SEMI_TONE);
        assert_eq!(Tone::from(2), TONE);
        assert_eq!(Tone::from(3), Tone(3));
        assert_eq!(Tone::from(12), OCTAVE);
    }

    #[test]
    fn tone_to_u8() {
        assert_eq!(u8::from(SEMI_TONE), 1);
        assert_eq!(u8::from(TONE), 2);
        assert_eq!(u8::from(Tone(3)), 3);
        assert_eq!(u8::from(OCTAVE), 12);
    }
}
