use musika_rs::{chord, C};

fn main() {
    let chord = chord::Major::new(&C);
    println!("chord: {chord:?}");
}
