// Grouping Related Code in Modules

// mod front_of_house {
//     mod hosting {
//         fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }




// Paths for Referring to an Item in the Module Tree
// mod front_of_house {
//     mod hosting {
//         fn add_to_waitlist() {}
//     }
// }

// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();

//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
// }

// change privacy rules
// mod front_of_house {
//     // added pub keyword
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();

//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
// }


// Starting Relative Paths with super
// fn deliver_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::deliver_order();
//     }

//     fn cook_order() {}
// }


// Making Structs and Enums Public
// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }

//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }
// }

// pub fn eat_at_restaurant() {
//     // Order a breakfast in the summer with Rye toast
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     // Change our mind about what bread we'd like
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);

//     // The next line won't compile if we uncomment it; we're not allowed
//     // to see or modify the seasonal fruit that comes with the meal
//     // meal.seasonal_fruit = String::from("blueberries");
// }

// pub fn eat_at_restaurant() {
//     let order1 = back_of_house::Appetizer::Soup;
//     let order2 = back_of_house::Appetizer::Salad;
// }


// Bringing Paths into Scope with the use Keyword
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
// }



// Creating Idiomatic use Paths
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// use crate::front_of_house::hosting::add_to_waitlist;

// pub fn eat_at_restaurant() {
//     add_to_waitlist();
// }

// use std::collections::HashMap;

// fn main() {
//     let mut map = HashMap::new();
//     map.insert(1, 2);
// }


// Generally use paths with parent module to avoid name conflict
// use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {
//     // --snip--
// }

// fn function2() -> io::Result<()> {
//     // --snip--
// }

// Providing New Names with the as Keyword
// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function1() -> Result {
//     // --snip--
// }

// fn function2() -> IoResult<()> {
//     // --snip--
// }


// Re-exporting Names with pub use
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// pub use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
// }

// Using External Packages
// use rand::{ Rng, CryptoRng };

// fn main() {
//     let secret_number = rand::thread_rng().gen_range(1..=100);
// }


// Using Nested Paths to Clean Up Large use Lists
// use std::{cmp::Ordering, io};
// use std::io::{self, Write};


// The Glob Operator
// use std::collections::*;

// Separating Modules into Different Files
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}