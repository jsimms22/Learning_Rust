fn main() {
    // --- Options Enum ---

    /* Example of what Option looks like
    enum Option<T> {
        Some(T),
        None, //replaces the idea of NULL in Rust
    }
    */

    // examples of Option
    let some_number = Some(5);

    let some_string = Some("a string");

    let absent_number: Option<i32> = None; //cannot derive type if None is used, therefore requires explicit type declaration

    // using Option wrapped variables
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap_or(0); // Option has some useful methods we can call directly on it
}
