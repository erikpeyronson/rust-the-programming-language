fn main() {
    let some_option_value: Option<i32> = None;

    // This will not compile since let requires an irrefutable pattern
    // let Some(x) = some_option_value;

    // if let will compile however since it accepts refutable patterns
    if let Some(x) = some_option_value {
        println!("{}", x);
    }

    // Using irrefutable pattern in if let gives warning
    if let x = 5 {
        println!("{}", x);
    };
}
