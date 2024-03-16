use std::fmt::{Debug, Display};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Octave(u8);

impl Debug for Octave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let octave = self.0;
        write!(f, "C{octave}")
    }
}

impl Display for Octave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let octave = self.0;
        write!(f, "C{octave}")
    }
}

impl Octave {
    fn new(value: u8) -> Self {
        Self(value)
    }
}

pub const C0: Octave = Octave(0);
pub const C1: Octave = Octave(1);
pub const C2: Octave = Octave(2);
pub const C3: Octave = Octave(3);
pub const C4: Octave = Octave(4);
pub const C5: Octave = Octave(5);
pub const C6: Octave = Octave(6);
pub const C7: Octave = Octave(7);

pub const OCTAVE_DEFAULT: Octave = C4;

pub const OCTAVES: [&Octave; 8] = [&C0, &C1, &C2, &C3, &C4, &C5, &C6, &C7];
pub const OCTAVE_COUNT: u8 = OCTAVES.len() as u8;

impl Default for Octave {
    fn default() -> Self {
        C4
    }
}

impl From<Octave> for u8 {
    fn from(octave: Octave) -> Self {
        octave.0
    }
}

impl From<u8> for Octave {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(format!("{C0}"), "C0");
        assert_eq!(format!("{C1}"), "C1");
        assert_eq!(format!("{C2}"), "C2");
        assert_eq!(format!("{C3}"), "C3");
        assert_eq!(format!("{C4}"), "C4");
        assert_eq!(format!("{C5}"), "C5");
        assert_eq!(format!("{C6}"), "C6");
        assert_eq!(format!("{C7}"), "C7");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{C0:#}"), "C0");
        assert_eq!(format!("{C1:#}"), "C1");
        assert_eq!(format!("{C2:#}"), "C2");
        assert_eq!(format!("{C3:#}"), "C3");
        assert_eq!(format!("{C4:#}"), "C4");
        assert_eq!(format!("{C5:#}"), "C5");
        assert_eq!(format!("{C6:#}"), "C6");
        assert_eq!(format!("{C7:#}"), "C7");
    }

    #[test]
    fn octave_to_u8() {
        assert_eq!(u8::from(C0), 0);
        assert_eq!(u8::from(C1), 1);
        assert_eq!(u8::from(C2), 2);
        assert_eq!(u8::from(C3), 3);
        assert_eq!(u8::from(C4), 4);
        assert_eq!(u8::from(C5), 5);
        assert_eq!(u8::from(C6), 6);
        assert_eq!(u8::from(C7), 7);
    }

    #[test]
    fn u8_to_octave() {
        assert_eq!(Octave::from(0), C0);
        assert_eq!(Octave::from(1), C1);
        assert_eq!(Octave::from(2), C2);
        assert_eq!(Octave::from(3), C3);
        assert_eq!(Octave::from(4), C4);
        assert_eq!(Octave::from(5), C5);
        assert_eq!(Octave::from(6), C6);
        assert_eq!(Octave::from(7), C7);
    }
}
