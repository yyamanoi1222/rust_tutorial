fn serve_order() {}

mod back_of_house {
    pub enum Appetizer {}
    pub struct Breakfast {
        pub toast:String,
        seasonal_fruit:String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eeat_at_restaurant() {
    // abs
    crate::front_of_house::hosting::add_to_waitlist();

    // rel
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("test");
    meal.toast = String::from("a");
}
