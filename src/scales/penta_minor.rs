use super::Scales;
use crate::Note;

pub fn minor(tonic: Note) -> Scales {
    let steps = [3, 2, 2, 3];
    Scales::minor_with_steps(" penta_minor", tonic, steps.into_iter())
}

pub fn pentatonic_minor(tonic: Note) -> Scales {
    let steps = [3, 2, 2, 3, 2];
    Scales::minor_with_steps(" penta minor", tonic, steps.into_iter())
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{A, C};

    #[test]
    fn test_minor() {
        let scale = minor(C);
        assert_eq!(format!("{scale:X}"), "C minor [C, D, D#, F, G, G#, A#, C]");
        assert_eq!(format!("{scale:x}"), "C minor [C, D, Eb, F, G, Ab, Bb, C]");
    }

    #[test]
    fn test_penta_minor() {
        let scale = pentatonic_minor(A);
        assert_eq!(format!("{scale:X}"), "A penta minor [A, C, D, E, G, A]");
        assert_eq!(format!("{scale:x}"), "A penta minor [A, C, D, E, G, A]");

        let scale = pentatonic_minor(D);
        assert_eq!(format!("{scale:X}"), "D penta minor [D, E, F, G, B, D]");
        assert_eq!(format!("{scale:x}"), "D penta minor [D, E, F, G, B, D]");
    }
}
