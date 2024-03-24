use musika_rs::{scales, Bar, A, B, C, D, E, F, G};

fn exercise1() {
    println!();
    println!("Exercise 1:");

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

fn exercise2() {
    println!();
    println!("Exercise 2:");

    let rh_bar = Bar::new()
        .with_note(A, 8)
        .with_note(C, 8)
        .with_note(D, 8)
        .with_note(E, 8)
        .with_note(G, 8)
        .with_note(E, 8)
        .with_note(D, 8)
        .with_note(C, 8);

    println!();

    // Bar 1
    println!("RH: {rh_bar:X}");
    let lh_bar1 = Bar::new().with_chord(D.min7(), 1);
    println!("LH: {lh_bar1}");

    println!();

    // Bar 2
    println!("RH: {rh_bar:X}");
    let lh_bar2 = Bar::new().with_chord(G.dom7(), 1);
    println!("LH: {lh_bar2}");

    println!();

    // Bar 3
    println!("RH: {rh_bar:X}");
    let lh_bar3 = Bar::new().with_chord(C.maj7(), 1);
    println!("LH: {lh_bar3}");

    println!();

    // Bar 4
    println!("RH: {rh_bar:X}");
    let lh_bar4 = Bar::new().with_silence(1);
    println!("LH: {lh_bar4}");

    println!();
    println!("where:");
    println!("  {:X}", D.min7());
    println!("  {:X}", G.dom7());
    println!("  {:X}", C.maj7());
}

fn exercise3() {
    println!();
    println!("Exercise 3:");

    println!();

    // Bar 1
    let rh_bar1 = Bar::new()
        .with_note(D, 8)
        .with_note(F, 8)
        .with_note(G, 8)
        .with_note(A, 8)
        .with_note(C, 8);
    println!("RH: {rh_bar1:X}");
    let lh_bar1 = Bar::new().with_chord(D.min7(), 1);
    println!("LH: {lh_bar1}");

    println!();

    // Bar 2
    let rh_bar2 = Bar::new()
        .with_note(A, 8)
        .with_note(C, 8)
        .with_note(D, 8)
        .with_note(E, 8)
        .with_note(G, 8);
    println!("RH: {rh_bar2:X}");
    let lh_bar2 = Bar::new().with_chord(G.dom7(), 1);
    println!("LH: {lh_bar2}");

    println!();

    // Bar 3
    let rh_bar3 = Bar::new()
        .with_note(E, 8)
        .with_note(G, 8)
        .with_note(A, 8)
        .with_note(B, 8)
        .with_note(D, 8);
    println!("RH: {rh_bar3:X}");
    let lh_bar3 = Bar::new().with_chord(C.maj7(), 1);
    println!("LH: {lh_bar3}");

    println!();

    // Bar 4
    let rh_bar4 = Bar::new()
        .with_note(E, 8)
        .with_note(G, 8)
        .with_note(A, 8)
        .with_note(B, 8)
        .with_note(D, 8);
    println!("RH: {rh_bar4:X}");
    let lh_bar4 = Bar::new().with_chord(C.maj7(), 1);
    println!("LH: {lh_bar4}");

    println!();
}

fn main() {
    let scale = scales::pentatonic_minor(A);
    println!("{scale:X}");

    exercise1();
    exercise2();
    exercise3();
}
