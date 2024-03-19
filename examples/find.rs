use musika_rs::{chords::Chords, A, A_SHARP, C, C_SHARP, D, E, F, G};

fn main() {
    // let notes = [A, G, A_SHARP, C_SHARP, F];
    let notes = [C, E, G];
    let chords = Chords::find_contain_notes(A, notes.into_iter()).collect::<Vec<_>>();

    if chords.is_empty() {
        println!("Didn't find any chord!")
    } else {
        for chord in chords {
            println!("{chord:X}")
        }
    }
}
