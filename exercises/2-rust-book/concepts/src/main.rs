fn main() {
    // Variables and Mutability

    // Mutability
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Shadowing
    let y = 5;
    println!("The value of y is: {}", y);
    let y = "John Doe";
    println!("The value of y is: {}", y);

    // Constant
    const PI: f64 = 3.14159265359;
    println!("Unmutable constant: {}", PI);



    // Data Types
    // Scalar Data Types
    
    // Integers
    let decimal = 98_222; //Decimal
    let hex = 0xff; // Hex
    let octal = 0o77; // Octal
    let binary = 0b1111_0000; // Binary
    let byte = b'A'; // Byte (u8 only)

    // Floating-point numbers
    let f = 2.1;
    let g = 3.7;
    // addition
    let sum = 21 + 37;
    // substraction
    let difference = 21.37 - 4.20;
    // multiplication
    let product = 21 * 37;
    // division
    let quotient = 21 / 37;
    // remainder
    let remainder = 37 % 21;

    // Booleans
    let t = true;
    let f = false;

    // Character
    let shield = '🛡';



    // Compound Types

    // Tuples
    let tup = ("John Doe", 17);
        // destructurizing
        let (name, age) = tup;
        // dot notation
        let age = tup.1;
    
    // Arrays
    let error_codes = [200, 404, 500];
    let not_found = error_codes[1];

    let byte_ = [0; 8];

    let x_plus_2137 = my_function(x);
    println!("x_plus_2137: {}", x_plus_2137);


    // Control flow
    // if
    let number = 5;
    if number < 10 {
        println!("number less than 10")
    } else if number < 22{
        println!("number less than 22")
    } else {
        println!("number greater than 22")
    }

    let condition = true;
    let mut number = if condition { 5 } else { 6 };

    //loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        println!("counter {}", counter);
        if counter == 10 {
            break counter;
        }
    };

    //while 
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("result {}", result);

    //for
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in 1..4 {
        println!("{}!", number);
    }

    // Line Comment
    /* 
        Block Comment 
    */
}

// Functions

fn my_function(x: i32) -> i32 {
    println!("My function with arguments: {}", x);
    x + 2137
}