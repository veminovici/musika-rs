use super::Chords;
use crate::Note;

pub fn dom7(root: Note) -> Chords {
    let steps = [4, 3, 3];
    Chords::dominant_with_steps("7", root, steps.into_iter())
}

pub fn dom7b5(root: Note) -> Chords {
    let steps = [4, 2, 4];
    Chords::dominant_with_steps("7b5", root, steps.into_iter())
}

pub fn dom7s5(root: Note) -> Chords {
    let steps = [4, 4, 2];
    Chords::dominant_with_steps("7#5", root, steps.into_iter())
}

pub fn dom9(root: Note) -> Chords {
    let steps = [4, 3, 3, 4];
    Chords::dominant_with_steps("9", root, steps.into_iter())
}

pub fn dom11(root: Note) -> Chords {
    let steps = [4, 3, 3, 4, 3];
    Chords::dominant_with_steps("11", root, steps.into_iter())
}

pub fn dom13(root: Note) -> Chords {
    let steps = [4, 3, 3, 4, 3, 4];
    Chords::dominant_with_steps("13", root, steps.into_iter())
}

pub fn dom13b9b13(root: Note) -> Chords {
    let steps = [4, 3, 3, 3, 4, 3];
    Chords::dominant_with_steps("13b9b13", root, steps.into_iter())
}

pub fn dominant_chords(root: Note) -> impl Iterator<Item = Chords> {
    [
        dom7(root),
        dom7b5(root),
        dom7s5(root),
        dom9(root),
        dom11(root),
        dom13(root),
        dom13b9b13(root),
    ]
    .into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::C;

    #[test]
    fn test_dom7() {
        let chord = dom7(C);
        assert_eq!(format!("{chord:X}"), "C7 [C, E, G, A#]");
        assert_eq!(format!("{chord:x}"), "C7 [C, E, G, Bb]");
    }

    #[test]
    fn test_dom7b5() {
        let chord = dom7b5(C);
        assert_eq!(format!("{chord:X}"), "C7b5 [C, E, F#, A#]");
        assert_eq!(format!("{chord:x}"), "C7b5 [C, E, Gb, Bb]");
    }

    #[test]
    fn test_dom7s5() {
        let chord = dom7s5(C);
        assert_eq!(format!("{chord:X}"), "C7#5 [C, E, G#, A#]");
        assert_eq!(format!("{chord:x}"), "C7#5 [C, E, Ab, Bb]");
    }

    #[test]
    fn test_dom9() {
        let chord = dom9(C);
        assert_eq!(format!("{chord:X}"), "C9 [C, E, G, A#, D]");
        assert_eq!(format!("{chord:x}"), "C9 [C, E, G, Bb, D]");
    }

    #[test]
    fn test_dom11() {
        let chord = dom11(C);
        assert_eq!(format!("{chord:X}"), "C11 [C, E, G, A#, D, F]");
        assert_eq!(format!("{chord:x}"), "C11 [C, E, G, Bb, D, F]");
    }

    #[test]
    fn test_dom13() {
        let chord = dom13(C);
        assert_eq!(format!("{chord:X}"), "C13 [C, E, G, A#, D, F, A]");
        assert_eq!(format!("{chord:x}"), "C13 [C, E, G, Bb, D, F, A]");
    }
}
