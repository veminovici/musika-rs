use musika_rs::{Bar, A, B, C, D, E, F, G};

fn main() {
    let bar = Bar::new()
        .with_note(C, 8)
        .with_note(E, 8)
        .with_note(F, 8)
        .with_note(G, 8)
        .with_note(A, 8)
        .with_note(G, 8)
        .with_note(F, 8)
        .with_note(E, 8);
    println!("{bar:X}");

    let bar = Bar::new()
        .with_note(D, 8)
        .with_note(F, 8)
        .with_note(G, 8)
        .with_note(A, 8)
        .with_note(B, 8)
        .with_note(A, 8)
        .with_note(G, 8)
        .with_note(F, 8);
    println!("{bar:X}");
}
