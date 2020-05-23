fn main() {
    // Endless loop
    // loop {
    //     println!("Again!");
    // }

    // Assigning the value of a loop
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // While loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    // Looping over collection
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // While
    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }

    // For
    for element in a.iter() {
        println!("The value is: {}", element);
    }

    // Range
    for number in 1..4 {
        println!("{}", number);
    }
    // reverse range
    for number in (1..4).rev() {
        println!("{}", number);
    }
}
