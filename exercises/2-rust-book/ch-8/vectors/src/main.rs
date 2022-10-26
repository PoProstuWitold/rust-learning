// - vector allows you to store a variable number of values next to each other.
// - string is a collection of characters. We’ve mentioned the String type previously, 
// but in this chapter we’ll talk about it in depth.
// - hash map allows you to associate a value with a particular key. 
// It’s a particular implementation of the more general data structure called a map.


// Creating a New Vector
// fn main() {
//     let v: Vec<i32> = Vec::new();

//     // Updating a Vector
//     v.push(1);
//     v.push(2);
//     v.push(3);

//     let v = vec![1, 2, 3];
// }


// Reading Elements of Vectors
// fn main() {
//     let v = vec![1, 2, 3, 4, 5];

//     let third: &i32 = &v[2];
//     // println!("The third element is {}", third);

//     let third: Option<&i32> = v.get(422);
//     match third {
//         Some(third) => println!("The third element is {}", third),
//         None => println!("There is no third element."),
//     }
// }



// Iterating over the Values in a Vector
// fn main() {
//     let mut v = vec![1, 2, 3, 4, 5];
    
//     for i in &mut v {
//         *i+=50
//     }

//     for i in &v {
//         println!("{}", i);
//     }
// }



// Using an Enum to Store Multiple Types
fn main() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    match &row[1] {
        SpreadsheetCell::Int(i) => print!("{}", i),
        _ => print!("Not a integer")
    }
}



// Dropping a Vector Drops Its Elementsj
// fn main() {
//     {
//         let v = vec![1, 2, 3, 4];

//         // do stuff with v
//     } // <- v goes out of scope and is freed here
// }
