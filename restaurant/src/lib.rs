#[allow(dead_code)]
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("💡 Adding to waitlist...");
        }
        fn seat_at_table() {
            println!("💡 Seating add table...")
        }
    }
    mod serving {
        fn take_order() {
            println!("💡 Taking order...")
        }
        fn serve_order() {
            println!("💡 Serving order...")
        }
        fn take_payment() {
            println!("💡 Taking payment...")
        }
    }
}

#[allow(dead_code)]
mod back_of_house {
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

    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    println!("💡 Using absolute path");
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    println!("💡 Using relative path");
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("💡 I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
    let order1 = back_of_house::Appetizer::Soup;
    println!("💡 I'd like to order {:?}", order1);
}
