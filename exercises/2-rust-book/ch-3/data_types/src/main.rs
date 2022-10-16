fn main() {
    // Data Types
    let guess: u32 = "10".parse().expect("Not a number!");


    // Scalar Types
    // A scalar type represents a single value. Rust has four primary scalar types: 
    // integers, floating-point numbers, Booleans, and characters.


    // Integer Types
    // Length	Signed	Unsigned
    // 8-bit	i8	    u8
    // 16-bit	i16	    u16
    // 32-bit	i32	    u32
    // 64-bit	i64	    u64
    // 128-bit	i128	u128
    // arch	    isize	usize


    // Integer Literals
    // Number literals	    Example
    // Decimal	            98_222
    // Hex	                0xff
    // Octal	            0o77
    // Binary	            0b1111_0000
    // Byte (u8 only)	    b'A'


    // Floating-Point Types
    // let x = 2.0; // f64

    // let y: f32 = 3.0; // f32


    // Numeric Operations
    // addition
    // let sum = 5 + 10;

    // subtraction
    // let difference = 95.5 - 4.3;

    // multiplication
    // let product = 4 * 30;

    // division
    // let quotient = 56.7 / 32.2;
    // let floored = 2 / 3; // Results in 0

    // remainder
    // let remainder = 43 % 5;


    // The Boolean Type
    // let t = true;

    // let f: bool = false; // with explicit type annotation


    // The Character Type
    // let c = 'z';
    // let z: char = 'ℤ'; // with explicit type annotation
    // let heart_eyed_cat = '😻';


    // Compound Types
    // Compound types can group multiple values into one type. 
    // Rust has two primitive compound types: tuples and arrays.

    // The Tuple Type
    // let tup: (i32, f64, u8) = (500, 6.4, 1);


    // The Array Type
    // let a = [1, 2, 3, 4, 5];
    // let months = ["January", "February", "March", "April", "May", "June", "July",
    //   "August", "September", "October", "November", "December"];
    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    // let a = [3; 5];
}
