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
