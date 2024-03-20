use musika_rs::{scales, Bar, A, C, D, E, G};

fn main() {
    let scale = scales::pentatonic_minor(A);
    println!("{scale:X}");

    let rh_bar = Bar::new()
        .with_note(A, 8)
        .with_note(C, 8)
        .with_note(D, 8)
        .with_note(E, 8)
        .with_note(G, 8)
        .with_note(A, 8)
        .with_silence(8)
        .with_silence(8);

    println!();

    // Bar 1
    println!("RH: {rh_bar:X}");
    let lh_bar1 = Bar::new().with_chord(D.min7(), 1);
    println!("LH: {lh_bar1:X}");

    println!();

    // Bar 2
    println!("RH: {rh_bar:X}");
    let lh_bar2 = Bar::new().with_chord(G.dom7(), 1);
    println!("LH: {lh_bar2:X}");

    println!();

    // Bar 3
    println!("RH: {rh_bar:X}");
    let lh_bar3 = Bar::new().with_chord(C.maj7(), 1);
    println!("LH: {lh_bar3:X}");

    println!();

    // Bar 4
    println!("RH: {rh_bar:X}");
    let lh_bar4 = Bar::new().with_silence(1);
    println!("LH: {lh_bar4:X}");
}
