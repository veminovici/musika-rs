# Simplee > Musika

[![CI][ci-badge]][ci-url]
![GitHub top language][lang-badge]
[![License:MIT][license-badge]][license-url]
![GitHub code size in bytes][size-badge]
![GitHub last commit][last-commit-badge]
![GitHub watchers][watchers-badge]

## Description
A Rust crate for musical basic elements.

```rust
use musika_rs::{
    chords::{self},
    C, C_SHARP,
};

let chord = chords::Major::from(C);
println!("Major C: {chord:X}");

let chord = chords::Major7::from(C);
println!("Major7 C: {chord:X}");

let chord = chords::Major7::from(C_SHARP);
println!("Major7 C#: {chord:X}");

let chord = chords::Dominant7::from(C);
println!("Dom7 C: {chord:X}");
```

You can find more examples in the [examples][examples_folder] folder.

## Chords
The create allows you to build the following chords:
- [major](major_chord_url)
- [major7][major7_chord_url]
- [minor][minor_chord_url]
- [minor7][minor7_chord_url]
- [minor7b5][minor7b5_chord_url]
- [diminished][dim_chord_url]
- [diminished7][dim7_chord_url]
- [dominant7][dom7_chord_url]

You can find all the chords in the [chords][chords_folder] folder.

## Piano Exercises
You can find all piano exercises implemented in the [practices][practices_file] file. You can see the practices by running:
```bsh
cargo run --example practices
```

### Exercise 1 (| C - Am | F - G |)
```rust
use musika_rs::*;

println!("Exercise 1: (| C - Am | F - G |)");
println!("{:X}", C.major()); // C [C, E, G]
println!("{:X}", A.minor()); // Am [A, C, E]
println!("{:X}", F.major()); // F [F, A, C]
println!("{:X}", G.major()); // G [G, B, D]
```

### Exercise 2 (| C - G | Am - F | C - G | F - Em - Dm - C |)
```rust
println!("Exercise 2: (| C - G | Am - F | C - G | F - Em - Dm - C |)");
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
```

## Resources
- [Piano Chords][piano_chords_url]

## About
> Code designed and written on the beautiful island of [**Saaremaa**][estonia_url], Estonia.

[ci-badge]: https://github.com/veminovici/musika-rs/actions/workflows/ci.yml/badge.svg?branch=main
[ci-url]: https://github.com/veminovici/musika-rs/actions/workflows/ci.yml
[lang-badge]: https://img.shields.io/github/languages/top/veminovici/musika-rs
[license-badge]: https://img.shields.io/badge/License-MIT-yellow.svg
[license-url]: https://opensource.org/licenses/MIT
[size-badge]: https://img.shields.io/github/languages/code-size/veminovici/musika-rs
[last-commit-badge]: https://img.shields.io/github/last-commit/veminovici/musika-rs
[watchers-badge]: https://img.shields.io/github/watchers/veminovici/musika-rs
[piano_chords_url]: https://www.pianochord.org/
[estonia_url]: https://goo.gl/maps/DmB9ewY2R3sPGFnTA
[examples_folder]: ./examples/
[practices_file]: ./examples/practices.rs
[chords_folder]: ./src/chords/
[major_chord_url]: https://www.pianochord.org/c-major.html
[major7_chord_url]: https://www.pianochord.org/cmaj7.html
[minor_chord_url]: https://www.pianochord.org/cm.html
[minor7_chord_url]: https://www.pianochord.org/cm7.html
[minor7b5_chord_url]: https://www.pianochord.org/cm7b5.html
[dim_chord_url]: https://www.pianochord.org/c-dim.html
[dim7_chord_url]: https://www.pianochord.org/c-dim7.html
[dom7_chord_url]: https://www.pianochord.org/c7.html