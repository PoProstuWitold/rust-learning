// // The Borrow Checker
// // fn main() {
// //     let r;                // ---------+-- 'a
// //                           //          |
// //     {                     //          |
// //         let x = 5;        // -+-- 'b  |
// //         r = &x;           //  |       | // x doesn't live long enough 
// //     }                     // -+       |
// //                           //          |
// //     println!("r: {}", r); //          |  

// //     {
// //         let x = 5;            // ----------+-- 'b
// //                               //           |
// //         let r = &x;           // --+-- 'a  |
// //                               //   |       |
// //         println!("r: {}", r); //   |       |
// //                               // --+       |
// //     }                         // ----------+
// // }


// // Generic Lifetimes in Functions
// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";

//     let result = longest(string1.as_str(), string2);
//     println!("The longest string is {}", result);
// }

// // &i32          // a reference
// // &'a i32       // a reference with an explicit lifetime
// // &'a mut i32   // a mutable reference with an explicit lifetime

// fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// Lifetime Annotations in Struct Definitions

// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

// fn main() {
//     let novel = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence = novel.split('.').next().expect("Could not find a '.'");
//     let i = ImportantExcerpt {
//         part: first_sentence,
//     };
// }


// Lifetime Elision
// fn main() {

// }

// 1. Each parameter that is a reference gets its own lifetime parameter

// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters

// 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method,
// the lifetime of self is assigned to all output lifetime parameters.

// fn first_word<'a>(s: &'a str) -> &'a str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }


// Lifetime Annotations in Method Definitions
// impl<'a> ImportantExcerpt<'a> {
//     fn level(&self) -> i32 {
//         3
//     }
// }

// impl<'a> ImportantExcerpt<'a> {
//     fn announce_and_return_part(&self, announcement: &str) -> &str {
//         println!("Attention please: {}", announcement);
//         self.part
//     }
// }

// fn main() {
//     let novel = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence = novel.split('.').next().expect("Could not find a '.'");
//     let i = ImportantExcerpt {
//         part: first_sentence,
//     };
// }

// The Static Lifetime
// let s: &'static str = "I have a static lifetime.";

// Generic Type Parameters, Trait Bounds, and Lifetimes Together
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}