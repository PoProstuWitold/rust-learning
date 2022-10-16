// Defining an Enum
enum IpAddrKind {
    V4,
    V6,
}


// Enum Values
// We can create instances of each of the two variants of IpAddrKind like this:
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

fn route(ip_kind: IpAddrKind) {}
// And we can call this function with either variant:
// route(IpAddrKind::V4);
// route(IpAddrKind::V6);

fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}


// The Option Enum and Its Advantages Over Null Values
// This enum is Option<T>, and it is defined by the standard library as follows:
// enum Option<T> {
//     None,
//     Some(T),
// }
