use musika_rs::C;

fn main() {
    let chord = C.maj();
    println!("{chord:X}");

    let chord = C.maj7();
    println!("{chord:X}");

    let chord = C.min();
    println!("{chord:X}");

    let chord = C.dom7();
    println!("{chord:X}");
}
