fn main() {
    // Function
    println!("Hello, world!");
    another_function(32, 64);

    // Statements and expression
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {:?}", y);

    println!("The value of five() is: {}", five());

    println!("The value of plus_one(5): {}", plus_one(5));
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}, the value of y is: {}", x, y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
