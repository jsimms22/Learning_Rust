/*
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(us_state) => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // match functions are exhaustive, all possible outcomes must be addressed
    // therefore this w/o the 'None' or a 'other' arm the code will not compile
    // 'other' can be treated as a catch-all term, and must be last in match
    match x {
        None => None,
        Some(i) => Some(i + 1),
        // other => move_player(other)
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}


// Use of 'other' more explicitly
/* fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
}*/

// Use of a 'unit value' or 'empty tuple' as a catch all
/* fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
}*/
*/
