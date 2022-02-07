struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


// Method Syntax
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


// Associated Functions
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("witold@mail.com"),
        username: String::from("witq2137"),
        sign_in_count: 1,
        active: true
    };

    let name = user1.username;
    user1.username = String::from("PoProstuWitold");

    let user2 = build_user(
        String::from("witq@mail.com"), 
        String::from("witq"
    ));

    let user3 = User {
        email: String::from("wojtek@mail.com"),
        username: String::from("wojtek"),
        ..user2
    };

    // Tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);


    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 40,
        height: 50,
    };

    let rect4 = Rectangle::square(32);

    println!("rect1 can hold: {}", rect1.can_hold(&rect2));
    println!("rect1 can hold: {}", rect1.can_hold(&rect3));
    println!("rect1: {:#?}", rect1);
    println!("rect4: {:#?}", rect4);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    )
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}