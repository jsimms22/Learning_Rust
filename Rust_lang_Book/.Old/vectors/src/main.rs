// https://doc.rust-lang.org/std/vec/struct.Vec.html
fn main() {
    // Vec<T>, where T = datatype stored
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Rust provides a macro to infer types, default integer type is i32
    let v2 = vec![1, 2, 3];

    let third : &i32 =  &v[2];
    println!("The third element is {}.", third);

    v.push(4);
    let mut last =  &v[(v.len()-1)];
    println!("The new last element is {} now.", last);

    v.push(10);
    last =  &v[(v.len()-1)];
    println!("The new last element is {} now.", last);

    match v.get(2) {
        Some(third) => println!("The third element is {}.",third),
        None => println!("There is no third element."),
    }

    // iterating through v and performing action
    println!("Printing vec v with a for-loop.");
    for i in &v {
        println!{"{}",i};
    }

    // iterating over all references in v and adding 50
    println!("Adding 50 to all values in vec v.");
    for i in &mut v {
        *i += 50;
    }

    println!("Printing modified vec v.");
    for i in &v {
        println!{"{}",i};
    }

    // using enums to store values of different type in a vector
    // can be grouped with a match to handle all cases to avoid breaking at compile time
    // rust compiler needs to know how much memory to allocate at compile time
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
} // vectors lose scope like other structs
