use musika_rs::*;

/// [Resource](https://www.youtube.com/watch?v=WrLFCznbNMw)
fn main() {
    println!();
    println!("Exercise 5: | Dm9x2 | G13x2 | Cmaj9x2 | Fmaj13X2 | A7 | Dm | G | C |");

    println!("{:X}", D.min9()); // D - F - A - C - E
    println!("{:X}", D.min9()); // D - F - A - C - E
    println!("{:X}", G.dom9()); // G - F - A - B - E
    println!("{:X}", C.maj9()); // C - E - G - B - D
                                // TODO: Find the Fmaj13 chord (4:48)
    println!("{:X}", F.maj9()); // F - E - G - A - D
                                // TODO: Find the A7 b5 b13
    println!("{:X}", A.dom9()); // G - F - A - B - E
}
