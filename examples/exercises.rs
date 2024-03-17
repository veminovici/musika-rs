use musika_rs::*;

fn exercise1() {
    println!();
    println!("Exercise 1: | C - Am | F - G |");
    println!("{:X}", C.major()); // C [C, E, G]
    println!("{:X}", A.minor()); // Am [A, C, E]
    println!("{:X}", F.major()); // F [F, A, C]
    println!("{:X}", G.major()); // G [G, B, D]
}

fn exercise2() {
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

fn exercise3() {
    println!();
    println!("Exercise 3: | Cx4 | Gx4 | Gx4 | Cx4 | Fx4 | Cx4 | Gx4 | Cx4 |");

    let cmaj = C.major();
    let gmaj = G.major();
    let fmaj = F.major();

    // bar 1
    for _ in 0..4 {
        println!("{:X}", cmaj); // C [C, E, G]
    }

    // bar 2
    for _ in 0..4 {
        println!("{:X}", gmaj); // C [C, E, G]
    }

    // bar 3
    for _ in 0..4 {
        println!("{:X}", gmaj); // C [C, E, G]
    }

    // bar 4
    for _ in 0..4 {
        println!("{:X}", cmaj); // C [C, E, G]
    }

    // bar 5
    for _ in 0..4 {
        println!("{:X}", fmaj); // C [C, E, G]
    }

    // bar 6
    for _ in 0..4 {
        println!("{:X}", cmaj); // C [C, E, G]
    }

    // bar 7
    for _ in 0..4 {
        println!("{:X}", gmaj); // C [C, E, G]
    }

    // bar 8
    for _ in 0..4 {
        println!("{:X}", cmaj); // C [C, E, G]
    }
}

fn exercise4() {
    println!();
    println!("Exercise 4: | C | F | Bdim | Em | Am | Dm | G | C |");

    println!("{:X}", C.major());
    println!("{:X}", F.major());
    println!("{:X}", B.diminished());
    println!("{:X}", E.minor());
    println!("{:X}", A.minor());
    println!("{:X}", D.minor());
    println!("{:X}", G.major());
    println!("{:X}", C.major());
}

fn main() {
    exercise1();
    exercise2();
    exercise3();
    exercise4();
}
