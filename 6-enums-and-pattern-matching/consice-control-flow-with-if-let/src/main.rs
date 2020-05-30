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
    let some_u8_value = Some(0u8);
    // Match with only one value
    match some_u8_value {
        Some(3) => println!("three"),
        _ => println!("Not three"),
    }

    // Can be written with if let
    if let Some(3) = some_u8_value {
        println!("Three");
    } else {
        println!("Not three")
    }
    let mut count = 0;
    for coin in generate_coins().iter() {
        if let Coin::Quarter(state) = coin {
            println!("State quarter from {:?}", state);
        } else {
            count += 1;
        }
    }
    println!("There are {}, non quarters in the list", count);
}

fn generate_coins() -> Vec<Coin> {
    let mut vec = Vec::new();
    vec.push(Coin::Nickel);
    vec.push(Coin::Penny);
    vec.push(Coin::Quarter(UsState::Alabama));
    vec
}
