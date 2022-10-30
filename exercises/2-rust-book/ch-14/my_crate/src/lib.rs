// Commenting Contained Items
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.
// Making Useful Documentation Comments
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

// Commonly Used Sections
// - Panics: The scenarios in which the function being documented could panic. 
// Callers of the function who don’t want their programs to panic should make 
// sure they don’t call the function in these situations.
// - Errors: If the function returns a Result, describing the kinds of errors 
// that might occur and what conditions might cause those errors to be returned 
// can be helpful to callers so they can write code to handle the different kinds of errors in different ways.
// - Safety: If the function is unsafe to call (we discuss unsafety in Chapter 19), 
// there should be a section explaining why the function is unsafe and covering 
// the invariants that the function expects callers to uphold.

// Documentation Comments as Tests
// cargo test runs docs as tests


// Exporting a Convenient Public API with pub use
// // ! # Art
// // !
// // ! A library for modeling artistic concepts.

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        match (c1, c2) {
            (Red, Blue) => SecondaryColor::Orange,
            _ => SecondaryColor::Orange
        }
    }
}