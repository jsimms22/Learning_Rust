fn main() {
    // ----- Ownership Rules ----- //
    // 1. Each value in Rust has a variable that's called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    // Example:
    { // s is not valid here, it's not yet declared
        let s = "hello"; // allocates string to the stack
        // do stuff with s
    } // this scope is now over, and s is no longer valid

    let x = 5;
    let y = x;

    let s1 = String::from("hello"); // allocate string to the heap
    // let s2 = s1; // Move, not a shallow copy -> this causes an error in the println below
    let s2 = s1.clone();

    println!("{}, world", s1);

    // Next Example:
    let s = String::from("hello");
    // takes_ownership(s); // This is also a Move, not a copy
    takes_ownership(s.clone());
    println!("{}", s); 

    // Next Example:
    let x = 5;
    makes_copy(x); // Integers are copied, not moved
    println!("{}", x);

    // Next Example:
    let s1 = gives_ownership(); // s1 takes onwership of the value returned by the function
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // this takes ownership of s2's value and returns it s3
    println!("s1 = {}, s3 = {}", s1, s3);

    // Next Example:
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // this is a borrow by reference, and does not transfer ownership of the value held by s1
    println!("The length of '{}' is {}", s1, len);

    let mut s1 = String::from("hello");
    change(&mut s1);
    println!("{}", s1);

    // Next Example:
    let mut s = String::from("hello");

    /* simulanteous ownership of a reference within the same scope to a mutable borrow is not allowed
    let r1 = &mut s;
    let r2 = &mut s;
    */

    // but simulanteous ownership to an immutable reference is allowed
    // you CANNOT mix mutable and immutable references
    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s; // NOT ALLOWED WHILE ABOVE REFERENCES EXIST IN SCOPE

    println!("{}, {}", r1, r2); //r1 and r2 scope ends here, last used here

    let r3 = &mut s;
    println!("{}", r3);

    // Next Example
    // let reference_to_nothing = dangle(); // the return for dangle leaves scope when terminating the function

    // The Rules of References
    // 1. At any given time, you can either have one mutable reference
    // or any number of immutable references
    //
    // 2. References must always be valid (no dangling)

    // Next Example
    let mut s = String::from("hello world");
    let hello = &s[..5]; // string slice, first index to index 4
    let world = &s[6..]; // string slice, sixth index to last index
    let hello_world = &s[..]; // string slice, full String

    let word = first_word(&s); // this is a now a string slice tied to the original String being analyzed by first_word
    //s.clear(); // we cannot clear the String now since there is a mutable (.clear()) and immutable (string slice) reference to s

    println!("the first word is: {}", word);

    // Previous Example Improvement
    let mut s = String::from("hello world");
    let s2 = "hello world";

    let word = first_word(&s2);

    println!("the first word is: {}", word);

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

fn takes_and_gives_back (a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize { // s is a reference to a string, and does not take ownership of the value owned by the input
    // s.push_str("oops"); // references are immutable by default
    let length = s.len(); // len() returns the length of a String
    length
} // s drops here

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/*
fn dangle() -> &String { // attempting to point to invalid memory for the return
    let s = String::from("hello");

    &s
}
*/

fn first_word(s: &str) -> &str { // we changed input type to a string slice for the example improvement
    let bytes = s.as_bytes(); // we take the string and convert to an array of bytes

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}