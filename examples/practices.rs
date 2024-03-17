use musika_rs::*;

fn exercise() {
    println!("Exercise 1: (C - Am - F - G");
    println!("{:X}", chords::Major::from(C)); // C
    println!("{:X}", chords::Minor::from(A)); // Amin
    println!("{:X}", chords::Major::from(F)); // F
    println!("{:X}", chords::Major::from(G)); // G
}

fn main() {
    exercise();
}
