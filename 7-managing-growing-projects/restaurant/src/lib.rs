// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}
       pub  fn serve_order() {}
        fn take_payment() {}
    }
}

mod back_of_house {
    // Struct fields are must be explicitly declared public 
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // Enum fields are automatically public
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order;
        super::front_of_house::serving::serve_order();
    }
    fn cook_order() {}
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // relative path
    front_of_house::hosting::seat_at_table();
    // Using "use"
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    println!("I'd like {} toast please", meal.toast);

    // Wont compile sinse seasonal_frut is not public
    // meal.seasonal_fruit = String::from("Blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
