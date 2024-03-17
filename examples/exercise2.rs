use musika_rs::*;

fn main() {
    println!();
    println!("Exercise 2: | C - G | Am - F | C - G | F - Em - Dm - C |");
    println!("{:X}", C.major()); // C [C, E, G]
    println!("{:X}", G.major()); // G [G, B, D]
    println!("{:X}", A.minor()); // Am [A, C, E]
    println!("{:X}", F.major()); // F [F, A, C]
    println!("{:X}", C.major()); // C [C, E, G]
    println!("{:X}", G.major()); // G [G, B, D]
    println!("{:X}", F.major()); // F [F, A, C]
    println!("{:X}", E.minor()); // Em [E, G, B]
    println!("{:X}", D.minor()); // Dm [D, F, A]
    println!("{:X}", C.major()); // C [C, E, G]
}
