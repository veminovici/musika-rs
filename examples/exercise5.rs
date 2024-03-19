use musika_rs::*;

fn key_c_major() {
    // Bar1
    let bar1 = Bar::new().with_chord(D.min9(), 2).with_chord(D.min9(), 2);
    let bar2 = Bar::new().with_chord(G.dom13(), 2).with_chord(G.dom13(), 2);

    let line1 = [&bar1, &bar2, &bar1, &bar2];
    let s = line1
        .iter()
        .map(|b| format!("{}", b))
        .collect::<Vec<_>>()
        .as_slice()
        .join(" | ");
    println!("| {s} |");

    let bar3 = Bar::new().with_chord(C.maj9(), 2).with_chord(C.maj9(), 2);
    let bar4 = Bar::new().with_chord(F.maj13(), 2).with_chord(F.maj13(), 2);
    let bar5 = Bar::new()
        .with_chord(A.dom13b9b13(), 2)
        .with_chord(A.dom13b9b13(), 2);

    let line2 = [&bar3, &bar4, &bar3, &bar5];
    let s = line2
        .iter()
        .map(|b| format!("{}", b))
        .collect::<Vec<_>>()
        .as_slice()
        .join(" | ");
    println!("| {s} |");
}

fn key_f_major() {
    // Bar1
    // D -> G [D, F, G]
    // G -> C [G, A, B]

    let bar1 = Bar::new().with_chord(G.min9(), 2).with_chord(G.min9(), 2);
    let bar2 = Bar::new().with_chord(C.dom13(), 2).with_chord(G.dom13(), 2);

    let line1 = [&bar1, &bar2, &bar1, &bar2];
    let s = line1
        .iter()
        .map(|b| format!("{}", b))
        .collect::<Vec<_>>()
        .as_slice()
        .join(" | ");
    println!("| {s} |");

    // C -> F [C, D, E, F]
    // F -> Bb
    let bar3 = Bar::new().with_chord(C.maj9(), 2).with_chord(C.maj9(), 2);
    let bar4 = Bar::new()
        .with_chord(A_SHARP.maj13(), 2)
        .with_chord(A_SHARP.maj13(), 2);
    // A -> D [A, B, C, D]
    let bar5 = Bar::new()
        .with_chord(D.dom13b9b13(), 2)
        .with_chord(D.dom13b9b13(), 2);

    let line2 = [&bar3, &bar4, &bar3, &bar5];
    let s = line2
        .iter()
        .map(|b| format!("{}", b))
        .collect::<Vec<_>>()
        .as_slice()
        .join(" | ");
    println!("| {s} |");
}

/// [Resource](https://www.youtube.com/watch?v=WrLFCznbNMw)
/// | Dm9 Dm9 | G13 G13 | Dm9 Dm9 | G13 G13 |
/// | Cmaj9 Cmaj9 | Fmaj13 Fmaj13 | Cmaj9 Cmaj9 | A7b9b13 |
fn main() {
    println!("Exercise 5: | Dm9x2 | G13x2 | Cmaj9x2 | Fmaj13X2 | A7 | Dm | G | C |");

    println!("Key: C");
    key_c_major();
    println!("Key: F");
    key_f_major();
}
