// fn main() {
//     let a = [1, 2, 3, 4, 5];
//     let mut v: Vec<i32> = Vec::new();

//     v.push(1);
//     v.push(2);
//     v.push(3);

//     let v2 = vec![1, 2, 3, 4, 5];

//     // index > 4 => panic! index out of bounds
//     let third = &v[2];
//     println!("The third element is {}", third);

//     // no panic!
//     match v2.get(2137) {
//         Some(index) => println!("The third element is {}", index),
//         None => println!("There is no such element")
//     }

//     let mut v3 = vec![1, 2, 3, 4, 5];

//     for i in &mut v3 {
//         *i += 50;
//     }

//     for i in &v3 {
//         println!("{}", i);
//     }


//     enum SpreadsheetCell {
//         Int(i32),
//         Float(f64),
//         Text(String),
//     }

//     let row = vec![
//         SpreadsheetCell::Int(3),
//         SpreadsheetCell::Text(String::from("blue")),
//         SpreadsheetCell::Float(10.12),
//     ];

//     match &row[1] {
//         SpreadsheetCell::Int(i) => println!("{}", i),
//         _ => println!("Not a integer")
//     }


//     // WHAT IS STRING?
//     // Strings are stored as a collection of UTF-8 encoded bytes
//     let s1 = String::new();
//     let s2 = "initial contents";
//     let s3 = s2.to_string();
//     let s4 = String::from("initial contents");

//     for c in "नमस्ते".chars() {
//         println!("{}", c);
//     }
    
// }
// use std::collections::HashMap;

// fn main() {
//     let blue = String::from("Blue");
//     let yellow = String::from("Yellow");

//     let mut scores = HashMap::new();

//     scores.insert(blue, 10);
//     scores.insert(yellow, 50);

//     let team_name = String::from("Blue");
//     let score = scores.get(&team_name);

//     for (key, value) in &scores {
//         println!("{}: {}", key, value);
//     }

//     scores.insert(String::from("Blue"), 10);

//     scores.entry(String::from("Yellow")).or_insert(50);
//     scores.entry(String::from("Blue")).or_insert(50);

//     println!("{:?}", scores);
// }
use std::collections::HashMap;

fn main() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}