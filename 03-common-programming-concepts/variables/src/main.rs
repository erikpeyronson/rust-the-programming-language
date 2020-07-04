fn main() {
    // Mutable variable
    let mut x = 5;
    println!("The value of X is: {}", x);
    x = 6;
    println!("The value of X is: {}", x);

    // Constants
    const MAX_POINTS: u32 = 100_000;

    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    // Shadowing
    let x = 5;

    let x = x + 1;
    let x = x * 2;

    println!("The value of X is: {}", x);

    let spaces = "    ";
    let spaces = spaces.len();

    // Not possible to change type of mutable variable
    // let mut spaces = "    ";
    // spaces = spaces.len();

    println!("The number of spaces is: {}", spaces);
}
