mod front_of_house {
    pub mod hosting { // exposing it to eat_at_restaurant
        pub fn add_to_wait_list() {}

        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}

        fn take_payment() {}
    }
}

fn serve_order() {}

mod back_of_house {
    fn cook_order() {}

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
        // super looks in the parent module
        // for the function
        // like a relative path
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    // pretend in this example that the
    // chef chooses the seasonal fruit,
    // but the customer gets to choose
    // the toast

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Salad,
        Soup,
    }
}

// all items are by default private
// child modules can use stuff inside
// parent modules, but not the other way
// around.

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_wait_list();

    // relative paths
    front_of_house::hosting::add_to_wait_list();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("wheat");

    println!("I'd like some {} toast", meal.toast);

    let order_1 = back_of_house::Appetizer::Soup;
    let order_2 = back_of_house::Appetizer::Salad;
}