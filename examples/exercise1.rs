use musika_rs::*;

fn main() {
    println!();
    println!("Exercise 1: | C - Am | F - G |");
    println!("{:X}", C.major()); // C [C, E, G]
    println!("{:X}", A.minor()); // Am [A, C, E]
    println!("{:X}", F.major()); // F [F, A, C]
    println!("{:X}", G.major()); // G [G, B, D]
}
