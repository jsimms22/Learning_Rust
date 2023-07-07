// crates can be treated as the root of all internal functions, etc
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_tables() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payments() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); //super can be used to create relative paths
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruits: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            toast: String::from(toast);
            seasonal_fruits: String::from("peaches");
        }
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
    
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    
    // Change our mind about what bread we'd like
    println!("I'd like {} toast please.", meal.toast);

    // The next line won't if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("Blueberries");
}
