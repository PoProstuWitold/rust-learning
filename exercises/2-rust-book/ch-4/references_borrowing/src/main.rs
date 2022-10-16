// References and Borrowing
// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize { // s is a reference to a String
//     s.len()
// } // Here, s goes out of scope. But because it does not have ownership of what
//   // it refers to, it is not dropped.

// Can't modify borrowed value
// fn main() {
//     let s = String::from("hello");

//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// Can't change the reference
// fn main() {
//     let s = String::from("hello");

//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }



// Mutable References
// Now we can change the reference
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// data race is similar to a race condition and happens when these three behaviors occur:
// 1. Two or more pointers access the same data at the same time.
// 2. At least one of the pointers is being used to write to the data.
// 3. There’s no mechanism being used to synchronize access to the data.
    // let mut s = String::from("hello");

    // {
    //     let r1 = &mut s;
    // } // r1 goes out of scope here, so we can make a new reference with no problems.

    // let r2 = &mut s;


// Rust enforces a similar rule for combining mutable and immutable references. This code results in an error:
// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     let r3 = &mut s; // BIG PROBLEM

//     println!("{}, {}, and {}", r1, r2, r3);
// }


// Dangling References
// dangling pointer--a pointer that references a location in memory that may have been given to someone else
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

// The solution here is to return the String directly:
// fn no_dangle() -> String {
//     let s = String::from("hello");

//     s
// }

// This works without any problems. Ownership is moved out, and nothing is deallocated.
// The Rules of References
// Let’s recap what we’ve discussed about references:
// 1. At any given time, you can have either one mutable reference or any number of immutable references.
// 2. References must always be valid.