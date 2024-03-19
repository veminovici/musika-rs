//! A crate for musical basic elements.
//!
//! # Example for chords
//!
//! ```
//! use musika_rs::{
//!     chords::{self},
//!     C, C_SHARP,
//! };
//!
//! let chord = chords::Major::from(C);
//! println!("Major C: {chord:X}");
//!
//! let chord = chords::Major7::from(C);
//! println!("Major7 C: {chord:X}");
//!
//! let chord = chords::Major7::from(C_SHARP);
//! println!("Major7 C#: {chord:X}");
//!
//! let chord = chords::Dominant7::from(C);
//! println!("Dom7 C: {chord:X}");
//! ```

mod bar;
pub mod chords;
mod note;
mod tone;
pub mod xchords;

pub use bar::*;
pub use note::*;
pub use tone::*;
