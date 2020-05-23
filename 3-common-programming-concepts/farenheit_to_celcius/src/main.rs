use std::io;
use std::io::Write;

// Exercise from the rust book, a simple program to convert between celcius and farenheit
// Only using concepts from chapter 1-3

fn main() {
    println!("Farenheit to celcius converter");

    io::stdout().flush().unwrap();

    loop {
        let temp = read_temp();
        let selection = read_selection();
        if selection == 'c' {
            let result = celcius_to_farenheit(temp);
            println!("{} celcius equal {} farenheit", temp, result);
        } else if selection == 'f' {
            let result = farenheit_to_celcius(temp);
            println!("{} farenheit equal {} celcius", temp, result);
        } else if selection == 'q' {
            break;
        } else {
            println!("Please enter a valid selection")
        }
    }
    println!("Good bye!")
}

fn farenheit_to_celcius(x: f32) -> f32 {
    (x - 32.0) * 5.0 / 9.0
}

fn celcius_to_farenheit(x: f32) -> f32 {
    9.0 * x / 5.0
}

fn read_temp() -> f32 {
    loop {
        print!("Input temperature: ");
        io::stdout().flush().unwrap();
        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Enter a valid string");

        match temp.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        };
    }
}

fn read_selection() -> char {
    print!("Convert to \n (c) Celcius \n (f) Farenheit\n (q) Quit\nSelection:",);
    io::stdout().flush().unwrap();
    let mut conv = String::new();
    io::stdin()
        .read_line(&mut conv)
        .expect("Failed to read line");
    let conv: char = conv.trim().parse().expect("Please enter a character");
    conv.to_ascii_lowercase()
}
