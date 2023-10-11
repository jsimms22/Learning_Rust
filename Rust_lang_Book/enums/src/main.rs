enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit, // stores no data
    Move { x: i32, y: i32}, // stores an anonymous struct
    Write (String), // stores a single String
    ChangeColor(i32, i32, i32), // store 3 x integers
}

impl Message {
    fn some_function() {
        println!{"Let's get Rusty!"};
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let localhost = IpAddrKind::V4(127, 0, 0, 1);
}

fn route(ip_kind: IpAddrKind) {}
