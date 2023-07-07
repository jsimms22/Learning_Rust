/*
#![allow(unused)]
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;

// Option allows variable to be something or nothing
// Rust does not have Nulls
// Option is included in the 'prelude'
/*enum Option<T> {
    None,
    Some(T),
}*/

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y;
    // The above line is invalid
    // i8 and and Option<i8> are different data types
}
*/