use musika_rs::*;

fn main() {
    println!();
    println!("Exercise 1: | C - Am | F - G |");
    println!("{:X}", C.maj()); // C [C, E, G]
    println!("{:X}", A.min()); // Am [A, C, E]
    println!("{:X}", F.maj()); // F [F, A, C]
    println!("{:X}", G.maj()); // G [G, B, D]
}
