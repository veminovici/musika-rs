use musika_rs::{Bar, A, B, C, D, E, F, G};

fn main() {
    let chord = E.dom7();
    println!("E7: {chord:X}");

    let bar0 = Bar::new().with_silence(1);
    let bar1 = Bar::new().with_chord(D.min7(), 1);
    let bar2 = Bar::new().with_chord(G.dom7(), 1);
    let bar3 = Bar::new().with_chord(C.maj7(), 1);
    let bar4 = Bar::new().with_chord(F.maj7(), 1);
    let bar5 = Bar::new().with_chord(B.min7b5(), 1);
    let bar6 = Bar::new().with_chord(E.dom7(), 1);
    let bar7 = Bar::new().with_chord(A.min7(), 1);
    let s = [bar0, bar1, bar2, bar3, bar4, bar5, bar6, bar7]
        .into_iter()
        .map(|bar| format!("{bar}"))
        .collect::<Vec<_>>()
        .join(" | ");
    println!("LH: | {s} |");
}
