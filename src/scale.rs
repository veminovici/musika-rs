use crate::{Note, Notes, NotesIterator, Tone, H, W};

pub struct Scale<const N: usize>(Notes<N>);

impl<const N: usize> Scale<N> {
    pub fn new(tonic: &Note, steps: [Tone; N]) -> Self {
        let notes = Notes::new(tonic, steps);
        Self(notes)
    }

    pub fn len(&self) -> usize {
        N + 1
    }

    pub fn tonic(&self) -> &Note {
        &self.0.root()
    }

    pub fn index(&self, idx: usize) -> Option<&Note> {
        match idx {
            0 => Some(&self.tonic()),
            n if n <= N => self.index(idx),
            _ => None,
        }
    }
}

impl Scale<7> {
    pub fn major(tonic: &Note) -> Self {
        let steps = [W, W, H, W, W, W, H];
        Self::new(tonic, steps)
    }
}

impl<const N: usize> IntoIterator for Scale<N> {
    type Item = Note;

    type IntoIter = NotesIterator<N>;

    fn into_iter(self) -> Self::IntoIter {
        NotesIterator::new(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{A, B, C, C5, D, E, F, G, H, W};

    #[test]
    fn new_scale() {
        let scale = Scale::new(&C, [W, W, H, W, W, W, H]);
        assert_eq!(scale.tonic(), &C);
        assert_eq!(scale.len(), 8);

        let mut iter = scale.into_iter();
        assert_eq!(iter.next(), Some(C));
        assert_eq!(iter.next(), Some(D));
        assert_eq!(iter.next(), Some(E));
        assert_eq!(iter.next(), Some(F));
        assert_eq!(iter.next(), Some(G));
        assert_eq!(iter.next(), Some(A));
        assert_eq!(iter.next(), Some(B));

        let note = iter.next().unwrap();
        assert_eq!(note.octave(), &C5);
        assert_eq!(note.note(), 0);

        assert!(iter.next().is_none());
    }

    #[test]
    fn major() {
        let scale = Scale::major(&C);
        assert_eq!(scale.tonic(), &C);
        assert_eq!(scale.len(), 8);

        let mut iter = scale.into_iter();
        assert_eq!(iter.next(), Some(C));
        assert_eq!(iter.next(), Some(D));
        assert_eq!(iter.next(), Some(E));
        assert_eq!(iter.next(), Some(F));
        assert_eq!(iter.next(), Some(G));
        assert_eq!(iter.next(), Some(A));
        assert_eq!(iter.next(), Some(B));

        let note = iter.next().unwrap();
        assert_eq!(note.octave(), &C5);
        assert_eq!(note.note(), 0);

        assert!(iter.next().is_none());
    }
}
