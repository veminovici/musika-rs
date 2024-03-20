use std::fmt::Display;

pub struct Interval(u8);

impl Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            1 => write!(f, "Minor2nd"),
            2 => write!(f, "Major2nd"),
            3 => write!(f, "Minor3rd"),
            4 => write!(f, "Major3rd"),
            5 => write!(f, "Perfect4th"),
            6 => write!(f, "Tritone"),
            7 => write!(f, "Perfect5th"),
            8 => write!(f, "Minor6th"),
            9 => write!(f, "Major6th"),
            10 => write!(f, "Minor7th"),
            11 => write!(f, "Major7th"),
            12 => write!(f, "Octave"),
            i => panic!("Interval: We should not be here [{i}]"),
        }
    }
}

pub const MINOR_2ND: Interval = Interval(1);
pub const MAJOR_2ND: Interval = Interval(2);

pub const MINOR_3RD: Interval = Interval(3);
pub const MAJOR_3RD: Interval = Interval(4);

pub const PERFECT_4TH: Interval = Interval(5);

pub const TRITONE: Interval = Interval(6);
pub const AUGMENTED_4TH: Interval = Interval(6);
pub const DIMINISHED_5TH: Interval = Interval(6);

pub const PERFECT_5TH: Interval = Interval(7);

pub const MINOR_6TH: Interval = Interval(8);
pub const MAJOR_6TH: Interval = Interval(9);

pub const MINOR_7TH: Interval = Interval(10);
pub const MAJOR_7TH: Interval = Interval(11);

pub const OCTAVE_8VE: Interval = Interval(12);

impl From<u8> for Interval {
    fn from(value: u8) -> Self {
        Self(value % 12)
    }
}

impl From<Interval> for u8 {
    fn from(interval: Interval) -> Self {
        interval.0
    }
}

impl AsRef<u8> for Interval {
    fn as_ref(&self) -> &u8 {
        &self.0
    }
}
