use super::Scales;
use crate::Note;

pub fn major(tonic: Note) -> Scales {
    let steps = [2, 2, 1, 2, 2, 2, 1];
    Scales::major_with_steps(" major", tonic, steps.into_iter())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::C;

    #[test]
    fn test_major() {
        let scale = major(C);
        assert_eq!(format!("{scale:X}"), "C major [C, D, E, F, G, A, B, C]");
        assert_eq!(format!("{scale:x}"), "C major [C, D, E, F, G, A, B, C]");
    }
}
