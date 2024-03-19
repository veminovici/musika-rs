use super::Chords;
use crate::Note;

pub fn dim(root: Note) -> Chords {
    let steps = [3, 3];
    Chords::diminished_with_steps("dim", root, steps.into_iter())
}

pub fn dim7(root: Note) -> Chords {
    let steps = [3, 3, 3];
    Chords::diminished_with_steps("dim7", root, steps.into_iter())
}

pub fn diminished_chords(root: Note) -> impl Iterator<Item = Chords> {
    [dim(root), dim7(root)].into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::C;

    #[test]
    fn test_dim() {
        let chord = dim(C);
        assert_eq!(format!("{chord:X}"), "Cdim [C, D#, F#]");
        assert_eq!(format!("{chord:x}"), "Cdim [C, Eb, Gb]");
    }

    #[test]
    fn test_dim7() {
        let chord = dim7(C);
        assert_eq!(format!("{chord:X}"), "Cdim7 [C, D#, F#, A]");
        assert_eq!(format!("{chord:x}"), "Cdim7 [C, Eb, Gb, A]");
    }
}
