use std::collections::HashMap;
use std::io;

fn main() {
    let integers = read_input();
    println!("Values: {:?}", integers);
    println!("Mean: {}", mean(&integers));
    println!("Median: {}", median(&integers));
    println!("Occurence count {:?}", count_occurences(&integers));
}

fn read_input() -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();
    loop {
        println!("Input a number use (q) to stop:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "q" {
            println!("Done");
            break;
        }

        let num: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };
        vec.push(num);
        vec.sort();
        println!("You entered: {}, All values: {:?}", num, &vec);
    }
    vec
}

fn mean(vec: &Vec<i32>) -> f64 {
    let mut count = 0;
    let mut sum = 0;
    for i in vec {
        count += 1;
        sum += i;
    }
    sum as f64 / count as f64
}

fn median(vec: &Vec<i32>) -> f32 {
    let len = vec.len();
    if vec.len() % 2 == 0 {
        (vec[len / 2 - 1] + vec[len / 2]) as f32 / 2.0
    } else {
        vec[len / 2] as f32
    }
}

fn count_occurences(vec: &Vec<i32>) -> HashMap<i32, i32> {
    let mut map = HashMap::new();
    for val in vec {
        let count = map.entry(*val).or_insert(0);
        *count += 1;
    }
    map
}
