use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tone(u8);

impl Display for Tone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            1 => write!(f, "H"),
            2 => write!(f, "W"),
            t => write!(f, "T({t})"),
        }
    }
}

impl From<Tone> for u8 {
    fn from(tone: Tone) -> Self {
        tone.0
    }
}

impl From<u8> for Tone {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

pub const H: Tone = Tone(1);
pub const W: Tone = Tone(2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(format!("{H}"), "H");
        assert_eq!(format!("{W}"), "W");
        assert_eq!(format!("{}", Tone(3)), "T(3)");
    }

    #[test]
    fn u8_to_tone() {
        assert_eq!(Tone::from(1), H);
        assert_eq!(Tone::from(2), W);
        assert_eq!(Tone::from(3), Tone(3));
    }

    #[test]
    fn tone_to_u8() {
        assert_eq!(u8::from(H), 1);
        assert_eq!(u8::from(W), 2);
        assert_eq!(u8::from(Tone(3)), 3);
    }
}
