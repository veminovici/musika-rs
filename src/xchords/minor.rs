use super::XChords;
use crate::Note;

pub fn min(root: Note) -> XChords {
    let steps = [3, 4];
    XChords::minor_with_steps("m", root, steps.into_iter())
}

pub fn min7(root: Note) -> XChords {
    let steps = [3, 4, 3];
    XChords::minor_with_steps("m7", root, steps.into_iter())
}

pub fn min7b5(root: Note) -> XChords {
    let steps = [3, 3, 4];
    XChords::minor_with_steps("m7(b5)", root, steps.into_iter())
}

pub fn min9(root: Note) -> XChords {
    let steps = [3, 4, 3, 4];
    XChords::minor_with_steps("m9", root, steps.into_iter())
}

pub fn min11(root: Note) -> XChords {
    let steps = [3, 4, 3, 4, 3];
    XChords::minor_with_steps("m11", root, steps.into_iter())
}

pub fn min13(root: Note) -> XChords {
    let steps = [3, 4, 3, 4, 3, 4];
    XChords::minor_with_steps("m13", root, steps.into_iter())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::C;

    #[test]
    fn test_min() {
        let chord = min(C);
        assert_eq!(format!("{chord:X}"), "Cm [C, D#, G]");
        assert_eq!(format!("{chord:x}"), "Cm [C, Eb, G]");
    }

    #[test]
    fn test_min7() {
        let chord = min7(C);
        assert_eq!(format!("{chord:X}"), "Cm7 [C, D#, G, A#]");
        assert_eq!(format!("{chord:x}"), "Cm7 [C, Eb, G, Bb]")
    }

    #[test]
    fn test_min7b5() {
        let chord = min7b5(C);
        assert_eq!(format!("{chord:X}"), "Cm7(b5) [C, D#, F#, A#]");
        assert_eq!(format!("{chord:x}"), "Cm7(b5) [C, Eb, Gb, Bb]")
    }

    #[test]
    fn test_min9() {
        let chord = min9(C);
        assert_eq!(format!("{chord:X}"), "Cm9 [C, D#, G, A#, D]");
        assert_eq!(format!("{chord:x}"), "Cm9 [C, Eb, G, Bb, D]")
    }

    #[test]
    fn test_min11() {
        let chord = min11(C);
        assert_eq!(format!("{chord:X}"), "Cm11 [C, D#, G, A#, D, F]");
        assert_eq!(format!("{chord:x}"), "Cm11 [C, Eb, G, Bb, D, F]")
    }

    #[test]
    fn test_min13() {
        let chord = min13(C);
        assert_eq!(format!("{chord:X}"), "Cm13 [C, D#, G, A#, D, F, A]");
        assert_eq!(format!("{chord:x}"), "Cm13 [C, Eb, G, Bb, D, F, A]")
    }
}
