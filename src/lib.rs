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
//! let chord = C.maj();
//! println!("C: {chord:X}");
//!
//! let chord = C.maj7();
//! println!("Cmaj7: {chord:X}");
//!
//! let chord = C.dom7();
//! println!("C7: {chord:X}");
//!
//! let chord = C.maj13();
//! println!("Cmaj13: {chord:X}");
//! ```

mod bar;
pub mod chords;
mod note;
pub mod scales;
mod tone;

pub use bar::*;
pub use note::*;
pub use tone::*;
