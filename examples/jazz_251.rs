use musika_rs::{Bar, C, D, G};

fn main() {
    let dm7 = D.min7();
    let g7 = G.dom7();
    let cmaj7 = C.maj7();

    let bar1 = Bar::new()
        .with_chord(dm7, 4)
        .with_silence(4)
        .with_silence(4)
        .with_silence(4);
    // println!("{bar1}");

    let bar2 = Bar::new()
        .with_chord(g7, 4)
        .with_silence(4)
        .with_silence(4)
        .with_silence(4);
    // println!("{bar2}");

    let bar3 = Bar::new()
        .with_chord(cmaj7, 4)
        .with_silence(4)
        .with_silence(4)
        .with_silence(4);
    // println!("{bar3}");

    let bar4 = Bar::new()
        .with_silence(4)
        .with_silence(4)
        .with_silence(4)
        .with_silence(4);
    // println!("{bar4}");

    let s = [bar1, bar2, bar3, bar4]
        .into_iter()
        .map(|b| format!("{b}"))
        .collect::<Vec<_>>()
        .join(" | ");
    println!("| {s} |");
}
