use musika_rs::{A, G_SHARP, OCTAVE, SEMI_TONE};

fn main() {
    let note = A;
    let i = i8::from(note);
    let b = note.base();
    let o = note.octave();
    println!("A: {i} => {note:?} base={b} octave={o}");

    let note = G_SHARP;
    let i = i8::from(note);
    let b = note.base();
    let o = note.octave();
    println!("G#: {i} => {note:?} base={b} octave={o}");

    let note = A + SEMI_TONE;
    let i = i8::from(note);
    let b = note.base();
    let o = note.octave();
    println!("A + H: {i} => {note:?} base={b} octave={o}");

    let note = G_SHARP + SEMI_TONE;
    let i = i8::from(note);
    let b = note.base();
    let o = note.octave();
    println!("G# + H: {i} => {note:?} base={b} octave={o}");

    let note = A - SEMI_TONE;
    let i = i8::from(note);
    let b = note.base();
    let o = note.octave();
    println!("A - H: {i} => {note:?} base={b} octave={o}");

    let note = A - SEMI_TONE - OCTAVE;
    let i = i8::from(note);
    let b = note.base();
    let o = note.octave();
    println!("A - H - O: {i} => {note:?} base={b} octave={o}");
}
