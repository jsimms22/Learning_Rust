fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_value = Some(3);
    match some_value {
        Some(3) => println!("three"),
        _ => (),
    }

    //match may be a bit verbose here

    // "if let" is not exhaustive and we can ignore other patterns
    if let Some(3) = some_value {
        println!("three");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        //None => None,
        Some(i) => Some(i + 1),
        _ => None, // the _ handles all other patterns not specified explicitly
    }
}
