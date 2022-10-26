// What Is a String?
// The String type, which is provided by Rust’s standard library 
// rather than coded into the core language, is a growable, mutable, 
// owned, UTF-8 encoded string type. When Rustaceans refer to “strings” in Rust, 
// they might be referring to either the String or the string slice &str types, 
// not just one of those types. Although this section is largely about String, 
// both types are used heavily in Rust’s standard library, 
// and both String and string slices are UTF-8 encoded.

// Creating a New String
fn main() {
    let mut s = String::new();
}


fn main() {
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
}

fn main() {
    let s = String::from("initial contents");
}

fn main() {
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}


// Updating a String
fn main() {
    let mut s = String::from("foo");
    s.push_str("bar");
}

fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
}


fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
}

fn add(self, s: &str) -> String {

fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
}


fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
}


// Indexing into Strings
fn main() {
    let s1 = String::from("hello");
    let h = s1[0];
}



// Internal Representation
// A String is a wrapper over a Vec<u8>
fn main() {
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}


// Bytes and Scalar Values and Grapheme Clusters! Oh My!



// Slicing Strings
fn main() {
    let hello = "Здравствуйте";

    let s = &hello[0..4];
}

// Methods for Iterating Over Strings
fn main() {
    for c in "Зд".chars() {
        println!("{}", c);
    }
}



// Strings Are Not So Simple