use super::Chords;
use crate::Note;

pub fn maj(root: Note) -> Chords {
    let steps = [4, 3];
    Chords::major_with_steps("", root, steps.into_iter())
}

pub fn maj7(root: Note) -> Chords {
    let steps = [4, 3, 4];
    Chords::major_with_steps("maj7", root, steps.into_iter())
}

pub fn maj9(root: Note) -> Chords {
    let steps = [4, 3, 4, 3];
    Chords::major_with_steps("maj9", root, steps.into_iter())
}

pub fn maj11(root: Note) -> Chords {
    let steps = [4, 3, 4, 3, 3];
    Chords::major_with_steps("maj11", root, steps.into_iter())
}

pub fn maj13(root: Note) -> Chords {
    let steps = [4, 3, 4, 3, 3, 4];
    Chords::major_with_steps("maj13", root, steps.into_iter())
}

pub fn major_chords(root: Note) -> impl Iterator<Item = Chords> {
    [maj(root), maj7(root), maj9(root), maj11(root), maj13(root)].into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::C;

    #[test]
    fn test_maj() {
        let chord = maj(C);
        assert_eq!(format!("{chord:X}"), "C [C, E, G]")
    }

    #[test]
    fn test_maj7() {
        let chord = maj7(C);
        assert_eq!(format!("{chord:X}"), "Cmaj7 [C, E, G, B]")
    }

    #[test]
    fn test_maj9() {
        let chord = maj9(C);
        assert_eq!(format!("{chord:X}"), "Cmaj9 [C, E, G, B, D]")
    }

    #[test]
    fn test_maj11() {
        let chord = maj11(C);
        assert_eq!(format!("{chord:X}"), "Cmaj11 [C, E, G, B, D, F]")
    }

    #[test]
    fn test_maj13() {
        let chord = maj13(C);
        assert_eq!(format!("{chord:X}"), "Cmaj13 [C, E, G, B, D, F, A]")
    }
}
