// crate is referencing itself, self <-> crate
// idiomatic to bring things into scope in Rust, we bring Parent function into scope as well
// pub is required if external code outside of this file wants to use the front_of_house::hosting
pub use self::front_of_house::{hosting};
use rand::{Rng, CryptoRng, ErrorKind::Transient}; // {} allows nested paths to remove common pathing

/* Example: 
use std::io;
use std::io::Write;
*/
use std::io::{self, Write};

//or

use std::io::*; // this brings all public paths into the scope of this file

// using ; instead of {} tells rust to pull the module definitions 
// from somewhere else with the same name
mod front_of_house; 

mod back_of_house {
    use crate::front_of_house::{serving};

    fn fix_incorrect_order() {
        cook_order();
        serving::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { 
                toast: String::from(toast), 
                seasonal_fruit: String::from("peaches"), 
            }
        }
    }

    //All variants in enums are Public if the encapsulating enum is Public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let secret_number = rand::thread_rng().gen_range(1, 100);

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    // Absolute Path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative Path
    front_of_house::hosting::add_to_waitlist();

    //fields within a struct are private by default
    let mut meal = 
        back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}


