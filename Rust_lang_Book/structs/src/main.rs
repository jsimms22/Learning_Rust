struct User {
    username: String, //we want the struct to own the string, not borrow via a slice
    email: String,
    sign_in_count: u64,
    active: bool,
}

// --- Tuple Struct --- //
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)] // allows us to println our struct in main()
struct Rectangle {
    width: u32,
    height: u32
}

// implementation methods house of methods for structs
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

//associated function, multiple impl sections allowed, but not required for impl vs associated
//associated do not pass self
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("john@mail.com"),
        username: String::from("john.smith"),
        sign_in_count: 1,
        active: true
    };

    let name = user1.username;
    user1.username = String::from("wallace123");

    let user2 = build_user(
        String::from("kyle@mail.com"), 
        String::from("kyle123")
    );

    let user3 = User {
        username: String::from("james@mail.com"), 
        email: String::from("james123"),
        ..user2
    };

    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    let rect2 = Rectangle {
        width: 20,
        height: 30
    };

    let rect3 = Rectangle::square(25);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!("rect: {:#?}", rect1); //:? debug, :#? new line each element

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("rect can hold rect1: {}", rect1.can_hold(&rect2));
    println!("rect can hold rect1: {}", rect1.can_hold(&rect3));
}

fn build_user(email: String, username: String) -> User {
    User {
        email, //field init shorthand syntax
        username,
        sign_in_count: 1,
        active: true,
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
    //dimensions.0 * dimensions.1
    //width * height
}