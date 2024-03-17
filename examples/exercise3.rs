use musika_rs::*;

fn main() {
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
