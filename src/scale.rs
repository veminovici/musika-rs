use crate::{Note, Notes, Tone, H, W};

pub struct Scale(Notes);

impl Scale {
    pub fn new(tonic: &Note, steps: impl Iterator<Item = Tone>) -> Self {
        let notes = Notes::new(tonic, steps);
        Self(notes)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn tonic(&self) -> &Note {
        self.0.root()
    }

    pub fn index(&self, idx: usize) -> &Note {
        self.0.index(idx)
    }

    pub fn major(tonic: &Note) -> Self {
        let steps = [W, W, H, W, W, W, H];
        Self::new(tonic, steps.into_iter())
    }
}

impl IntoIterator for Scale {
    type Item = Note;

    type IntoIter = <Vec<Note> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{A, B, C, C5, D, E, F, G, H, W};

    #[test]
    fn new_scale() {
        let scale = Scale::new(&C, [W, W, H, W, W, W, H].into_iter());
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
