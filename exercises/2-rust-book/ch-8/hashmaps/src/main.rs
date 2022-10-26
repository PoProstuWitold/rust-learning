// Creating a New Hash Map
// fn main() {
//     use std::collections::HashMap;

//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);
// }


// Accessing Values in a Hash Map
// fn main() {
//     use std::collections::HashMap;

//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);

//     let team_name = String::from("Blue");
//     let score = scores.get(&team_name).copied().unwrap_or(0);
// }


// We can iterate over each key/value pair in a hash map 
// in a similar manner as we do with vectors, using a for loop:
// fn main() {
//     use std::collections::HashMap;

//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);

//     for (key, value) in &scores {
//         println!("{}: {}", key, value);
//     }
// }


// Hash Maps and Ownership
// fn main() {
//     use std::collections::HashMap;

//     let field_name = String::from("Favorite color");
//     let field_value = String::from("Blue");

//     let mut map = HashMap::new();
//     map.insert(field_name, field_value);
//     // field_name and field_value are invalid at this point, try using them and
//     // see what compiler error you get!
// }


// Updating a Hash Map
// Overwriting a Value
// fn main() {
//     use std::collections::HashMap;

//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Blue"), 25);

//     println!("{:?}", scores);
// }

// Adding a Key and Value Only If a Key Isn’t Present
// fn main() {
//     use std::collections::HashMap;

//     let mut scores = HashMap::new();
//     scores.insert(String::from("Blue"), 10);

//     scores.entry(String::from("Yellow")).or_insert(50);
//     scores.entry(String::from("Blue")).or_insert(50);

//     println!("{:?}", scores);
// }


// Updating a Value Based on the Old Value
fn main() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}


// Hashing Functions
// By default, HashMap uses a hashing function called SipHash 
// that can provide resistance to Denial of Service (DoS) attacks involving hash tables1. 
// This is not the fastest hashing algorithm available, 
// but the trade-off for better security 
// that comes with the drop in performance is worth it.
// https://en.wikipedia.org/wiki/SipHash