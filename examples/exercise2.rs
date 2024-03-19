use musika_rs::*;

fn main() {
    println!();
    println!("Exercise 2: | C - G | Am - F | C - G | F - Em - Dm - C |");
    println!("{:X}", C.maj()); // C [C, E, G]
    println!("{:X}", G.maj()); // G [G, B, D]
    println!("{:X}", A.min()); // Am [A, C, E]
    println!("{:X}", F.maj()); // F [F, A, C]
    println!("{:X}", C.maj()); // C [C, E, G]
    println!("{:X}", G.maj()); // G [G, B, D]
    println!("{:X}", F.maj()); // F [F, A, C]
    println!("{:X}", E.min()); // Em [E, G, B]
    println!("{:X}", D.min()); // Dm [D, F, A]
    println!("{:X}", C.maj()); // C [C, E, G]
}
