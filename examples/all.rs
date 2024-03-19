use musika_rs::C;

fn main() {
    let root = C;
    for c in root.all_chords() {
        println!("{c:X}")
    }
}
