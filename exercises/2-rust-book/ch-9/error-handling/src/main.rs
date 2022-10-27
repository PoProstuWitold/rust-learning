// Unrecoverable Errors with panic!
// fn main() {
//     panic!("crash and burn");
// }

// Using a panic! Backtrace
// fn main() {
//     let v = vec![1, 2, 3];

//     v[99];
// }



// Recoverable Errors with Result
// use std::fs::File;

// fn main() {

//     // enum Result<T, E> {
//     //     Ok(T),
//     //     Err(E),
//     // }

//     let greeting_file_result = File::open("hello.txt");

//     let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => panic!("Problem opening the file: {:?}", error),
//     };
// }



// Matching on Different Errors
// use std::{ fs::File, io::ErrorKind, path::PathBuf, env };

// fn get_current_working_dir() -> std::io::Result<PathBuf> {
//     env::current_dir()
// }
// fn main() {
//     let working_dir = get_current_working_dir();
//     println!("current working dir: {:?}", working_dir);

//     let greeting_file_result = File::open("hello.txt");

//     let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating the file: {:?}", e),
//             },
//             other_error => {
//                 panic!("Problem opening the file: {:?}", other_error);
//             }
//         },
//     };

//     println!("File is: {:?}", greeting_file)
// }


// Shortcuts for Panic on Error: unwrap and expect
// use std::fs::File;

// fn main() {
//     let greeting_file = File::open("hello.txt").unwrap();
// }

// use std::fs::File;

// fn main() {
//     let greeting_file = File::open("hello.txt")
//         .expect("hello.txt should be included in this project");
// }


// Propagating Errors

// use std::fs::File;
// use std::io::{self, Read};

// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");

//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut username = String::new();

//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }

// fn main() {
//     let username = read_username_from_file();
//     println!("Username is {:?}", username.unwrap());
// }


// A Shortcut for Propagating Errors: the ? Operator
// use std::io;
// use std::fs;
// use std::io::Read;
// use std::fs::File;

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username_file = File::open("hello.txt")?;
//     let mut username = String::new();
//     username_file.read_to_string(&mut username)?;
//     Ok(username)
// }

// or

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username = String::new();

//     File::open("hello.txt")?.read_to_string(&mut username)?;

//     Ok(username)
// }

// or

// fn read_username_from_file() -> Result<String, io::Error> {
//     fs::read_to_string("hello.txt")
// }

// fn main() {
//     let username = read_username_from_file();
//     println!("Username is {:?}", username.unwrap());
// }


// Where The ? Operator Can Be Used
// fn last_char_of_first_line(text: &str) -> Option<char> {
//     text.lines().next()?.chars().last()
// }

// fn main() {
//     assert_eq!(
//         last_char_of_first_line("Hello, world\nHow are you today?"),
//         Some('d')
//     );

//     assert_eq!(last_char_of_first_line(""), None);
//     assert_eq!(last_char_of_first_line("\nhi"), None);
// }


use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> { //any type of error
    let _greeting_file = File::open("hello.txt")?;

    Ok(())
}
