use musika_rs::{
    chord::{self},
    C, C_SHARP,
};

fn main() {
    let chord = chord::Major::from(C);
    println!("Major C: {chord:X}");

    let chord = chord::Major7::from(C);
    println!("Major7 C: {chord:X}");

    let chord = chord::Major7::from(C_SHARP);
    println!("Major7 C#: {chord:X}");

    let chord = chord::Dominant7::from(C);
    println!("Dom7 C: {chord:X}");
}
