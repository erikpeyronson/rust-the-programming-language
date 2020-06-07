// Exercise from the rust book, a simple program to print the nth fibonachi number
// Only using concepts from chapter 1-3

use std::io;

fn main() {
    let index = read_number();
    let fib = fibonachi(index);
    println!(
        "The {}th number in the fibonachi sequence is {}",
        index, fib
    );
}

fn read_number() -> i32 {
    println!("Enter a fibonachi index:");
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Enter a valid string");
    num.trim().parse::<i32>().expect("Not a number")
}

fn fibonachi(index: i32) -> i32 {
    let mut n1 = 0;
    let mut n2 = 1;
    let mut nth = 0;
    for _ in 0..index {
        nth = n1 + n2;
        n1 = n2;
        n2 = nth;
    }
    nth
}
