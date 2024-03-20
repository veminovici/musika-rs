use musika_rs::{scales, A};

fn main() {
    let scale = scales::pentatonic_minor(A);
    println!("{scale:X}");
}
