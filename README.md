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

## Piano Exercises
You can find all piano exercises implemented in the [practices][practices_file] file. You can see the practices by running:
```bsh
cargo run --example practices
```

### Exercise 1 (C - Am - F - G)
```rust
use musika_rs::*;

println!("Exercise 1: (C - Am - F - G)");
println!("{:X}", C.major()); // C [C, E, G]
println!("{:X}", A.minor()); // Am [A, C, E]
println!("{:X}", F.major()); // F [F, A, C]
println!("{:X}", G.major()); // G [G, B, D]
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