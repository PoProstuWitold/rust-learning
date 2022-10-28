// Validating References with Lifetimes
// Preventing Dangling References with Lifetimes
// fn main() {
//     let r;

//     {
//         let x = 5;
//         r = &x;
//     }

//     println!("r: {}", r);
// }



// The Borrow Checker
// fn main() {
//     let r;                // ---------+-- 'a
//                           //          |
//     {                     //          |
//         let x = 5;        // -+-- 'b  |
//         r = &x;           //  |       |
//     }                     // -+       |
//                           //          |
//     println!("r: {}", r); //          |
// }                         // ---------+

// fn main() {
//     let x = 5;            // ----------+-- 'b
//                           //           |
//     let r = &x;           // --+-- 'a  |
//                           //   |       |
//     println!("r: {}", r); //   |       |
//                           // --+       |
// }                         // ----------+


// Generic Lifetimes in Functions
// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";

//     let result = longest(string1.as_str(), string2);
//     println!("The longest string is {}", result);
// }

// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// Lifetime Annotation Syntax
// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime



// Lifetime Annotations in Function Signatures
// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";

//     let result = longest(string1.as_str(), string2);
//     println!("The longest string is {}", result);
// }

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }


// Thinking in Terms of Lifetimes
// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "efghijklmnopqrstuvwxyz";

//     let result = longest(string1.as_str(), string2);
//     println!("The longest string is {}", result);
// }

// fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//     x
// }

// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

// --> src/main.rs:95:5
// |
// 95 |     result.as_str()
// |     ^^^^^^^^^^^^^^^ returns a reference to data owned by the current function


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
// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

// fn main() {
//     let my_string = String::from("hello world");

//     // first_word works on slices of `String`s
//     let word = first_word(&my_string[..]);

//     let my_string_literal = "hello world";

//     // first_word works on slices of string literals
//     let word = first_word(&my_string_literal[..]);

//     // Because string literals *are* string slices already,
//     // this works too, without the slice syntax!
//     let word = first_word(my_string_literal);
// }



// Lifetime Annotations in Method Definitions
// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

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
// fn main() {
//     let s: &'static str = "I have a static lifetime.";
// }


// Generic Type Parameters, Trait Bounds, and Lifetimes Together
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xydsaz";

    let result = longest_with_an_announcement(
        string1.as_str(),
        string2,
        "Today is someone's birthday!",
    );
    println!("The longest string is {}", result);
}

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
