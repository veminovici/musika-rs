use musika_rs::{
    chords::{self},
    C, C_SHARP,
};

fn main() {
    let chord = chords::Major::from(C);
    println!("Major C: {chord:X}");

    let chord = chords::Major7::from(C);
    println!("Major7 C: {chord:X}");

    let chord = chords::Major7::from(C_SHARP);
    println!("Major7 C#: {chord:X}");

    let chord = chords::Dominant7::from(C);
    println!("Dom7 C: {chord:X}");
}
