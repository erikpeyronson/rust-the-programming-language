#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // and so on
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn main() {
    let quarter = Coin::Quarter(UsState::Alabama);
    println!("quarter is {} cents", value_in_cents(quarter));
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five: {:?}, six: {:?}, none: {:?}", five, six, none);

    // _ will match anything nothing happens
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
