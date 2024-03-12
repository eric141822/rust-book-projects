//! # My crate_io
//! `crate_io` is a collection of examples to create a rust crate.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = crate_io::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Blue,
        Green,
    }
    /// The secondary colors according to the RYB color model.
    #[derive(Debug)]
    pub enum SecondaryColor {
        Orange,
        Purple,
        Green,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        match (c1, c2) {
            (PrimaryColor::Red, PrimaryColor::Blue) => SecondaryColor::Purple,
            (PrimaryColor::Red, PrimaryColor::Green) => SecondaryColor::Orange,
            (PrimaryColor::Blue, PrimaryColor::Red) => SecondaryColor::Purple,
            (PrimaryColor::Blue, PrimaryColor::Green) => SecondaryColor::Green,
            (PrimaryColor::Green, PrimaryColor::Red) => SecondaryColor::Orange,
            (PrimaryColor::Green, PrimaryColor::Blue) => SecondaryColor::Green,
            _ => panic!("Invalid color"),
        }
    }
}