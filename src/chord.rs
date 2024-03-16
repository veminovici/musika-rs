use crate::{Note, Notes, NotesIterator, Tone};

pub struct Chord<const N: usize>(Notes<N>);

impl<const N: usize> Chord<N> {
    pub fn new(root: &Note, steps: [Tone; N]) -> Self {
        let notes = Notes::new(root, steps);
        Self(notes)
    }

    pub fn len(&self) -> usize {
        N + 1
    }

    pub fn root(&self) -> &Note {
        &self.0.root()
    }

    pub fn index(&self, idx: usize) -> Option<&Note> {
        match idx {
            0 => Some(&self.root()),
            n if n <= N => self.index(idx),
            _ => None,
        }
    }
}

impl Chord<2> {
    pub fn major(root: &Note) -> Self {
        let steps = [Tone(4), Tone(3)];
        Self::new(root, steps)
    }

    pub fn minor(root: &Note) -> Self {
        let steps = [Tone(3), Tone(4)];
        Self::new(root, steps)
    }
}

impl<const N: usize> IntoIterator for Chord<N> {
    type Item = Note;

    type IntoIter = NotesIterator<N>;

    fn into_iter(self) -> Self::IntoIter {
        NotesIterator::new(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{A, C, E, G, O};

    #[test]
    fn major() {
        let chord = Chord::major(&C);
        assert_eq!(chord.root(), &C);
        assert_eq!(chord.len(), 3);

        let mut iter = chord.into_iter();
        assert_eq!(iter.next(), Some(C));
        assert_eq!(iter.next(), Some(E));
        assert_eq!(iter.next(), Some(G));
        assert!(iter.next().is_none());
    }

    #[test]
    fn minor() {
        let chord = Chord::minor(&A);
        assert_eq!(chord.root(), &A);
        assert_eq!(chord.len(), 3);

        let mut iter = chord.into_iter();
        assert_eq!(iter.next(), Some(A));
        assert_eq!(iter.next(), Some(C + O));
        assert_eq!(iter.next(), Some(E + O));
        assert!(iter.next().is_none());
    }
}
