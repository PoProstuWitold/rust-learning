fn main() {
    // Ownership rules for
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    // Variable scope
    {   // s is not valid here, it's not yet declared
        let _s = String::from("hello");
        // do stuff with s
    }   // the scope is over, s is no longer valid

    // Memory & Allocation
    let x = 5;
    let _y = x; // Copyright

    let s1 = String::from("hello");
    let _s2 = s1.clone();
    // let s2 = s1; // Move (not shallow copy)

    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s);

    let z = 5;
    makes_copy(z);
    println!("{}", z);

    let str1 = gives_ownership();
    let str2 = String::from("hello");
    let str3 = takes_and_gives_back_ownership(str2);

    println!("str1 = {}, str3 = {}", str1, str3);

    let str4 = String::from("hello");
    let length = calculate_length(&str4);
    println!("The length of {} the is {}", str4, length);

    let mut str5 = String::from("hello");
    change(&mut str5);
    println!("str5: {}", str5);


    // Rules of references
    // 1. At any given time, you can have either one mutable reference
    // or any number of immutable references.
    // 2. References must always be valid.


    // Slices
    let mut str6 = String::from("hello world");
    let str7 = "hello world";

    let word = first_word(&str7);


    let array = [1, 2, 3, 4, 5];
    let slice = &array[0..2];

}


fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back_ownership(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    let length = s.len(); // len() returns the length of the string
    length
}

fn change(some_string: &mut String) {
    some_string.push_str(", world")
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}