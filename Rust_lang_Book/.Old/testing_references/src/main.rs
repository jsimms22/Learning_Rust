fn main() {
    let s: i32 = 5;

    println!("printing 's': {}",s);
    println!("printing '&s': {}",&s);

    let s_ptr = &s;

    println!("printing 's': {}", s_ptr);
    println!("printing '&s': {}", &s_ptr);
}

//the following code does not work
fn main() {
    let reference_to_nothing = dangle();
}
    
fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String
    
    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!


// compilable version of the above code
fn main() {
    let reference_to_nothing = dangle();
}
    
fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String
    
    s // we return the String, s
} 
