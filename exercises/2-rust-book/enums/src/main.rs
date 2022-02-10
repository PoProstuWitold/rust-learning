// #[derive(Debug)]
// enum IpAddrKind {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// ENUMS
// impl Message {
//     fn some_function() {
//         println!("Hello world")
//     }
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // let localhost = IpAddrKind::V4(127, 0, 0, 1);

    // println!("localhost: {:?}", localhost);
// }


// fn route(ip_kind: IpAddrKind) {

// }

// OPTION ENUM
// enum Option<T> {
//     Some(T),
//     None
// }

// fn main() {
//     let some_number = Some(5);
//     let some_string = Some("a string");

//     // no value, so we have to annotate the type
//     let absent_number: Option<i32> = None;
// }

// fn main() {
//     let x: i8 = 5;
//     let y: Option<i8> = Some(5);

//     let sum = x + y.unwrap_or(0);
// }


// MATCH EXPRESSION
// fn main() {
//     value_in_cents(Coin::Quarter(UsState::Alaska));
// }

// #[derive(Debug)]
// enum UsState {
//     Alamaba,
//     Alaska,
//     Arizona,
//     Arkanas,
//     California,
//     // ...
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         },
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         },
//     }
// }

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => {
            println!("{} plus one", x.unwrap());
            Some(i + 1)
        }
    }
}