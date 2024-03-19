use musika_rs::*;

/// [Resource](https://www.youtube.com/watch?v=WrLFCznbNMw)
/// | Dm9 Dm9 | G13 G13 | Dm9 Dm9 | G13 G13 |
/// | Cmaj9 Cmaj9 | Fmaj13 Fmaj13 | Cmaj9 Cmaj9 | A7b9b13 |
fn main() {
    println!();
    println!("Exercise 5: | Dm9x2 | G13x2 | Cmaj9x2 | Fmaj13X2 | A7 | Dm | G | C |");

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
