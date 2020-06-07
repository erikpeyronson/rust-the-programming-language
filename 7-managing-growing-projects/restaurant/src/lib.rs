mod back_of_house;
mod front_of_house;

use crate::front_of_house::hosting;

// Compining pub and use makes the new name public
// pub use crate::front_of_house::hosting;

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
