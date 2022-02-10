//RUST_BACKTRACE=1 cargo run   
// fn main() {
//     let v = vec![1, 2, 3];

//     v[99];
// }
// use std::{ fs::File, io::ErrorKind };

// fn main() {
    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() { //panic!("Problem opening the file: {:?}", error),
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(file) => file,
    //             Err(error) => panic!("Problem creating the file: {:?}", error)
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     }
    // };

    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });

//     let f = File::open("hello.txt").expect("Failed to open hello.txt");
// }

// use std::fs::{self, File};
// use std::io::{self, Read};

// fn main() {
//     read_username_from_file();
// }

// fn read_username_from_file() -> Result<String, io::Error> {
//     // let mut s = String::new();
//     // File::open("hello.txt")?.read_to_string(&mut s)?;;
//     // Ok(s)
//     // let mut f = match f {
//     //     Ok(file) => file,
//     //     Err(e) => return Err(e),
//     // };

//     // match f.read_to_string(&mut s) {
//     //     Ok(_) => Ok(s),
//     //     Err(e) => Err(e),
//     // }
//     fs::read_to_string("hello.txt")
// }

use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}