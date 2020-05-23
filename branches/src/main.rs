fn main() {
    let number = 3;
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    let number = 7;
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    // Compile error
    // if number {
    //     println!(number);
    // }

    let number = 3;
    if number != 0 {
        println!("Number was something different than zero");
    }

    let number = 8;
    if number % 4 == 0 {
        println!("Number is dividable by four");
    } else if number % 3 == 0 {
        println!("Number is dividable by three");
    } else if number % 2 == 0 {
        println!("Number is dividable by two");
    } else {
        println!("Number is not dividable by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is {}", number);
}
