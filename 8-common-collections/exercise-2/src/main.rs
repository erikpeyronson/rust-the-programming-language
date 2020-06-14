use std::io;

fn main() {
    println!("Pig latin creator, enter a word:");
    let mut word = String::new();
    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read line");

    let mut chars = word.trim().chars().peekable();

    let mut piglatin_string = String::new();

    let suffix = match chars.next() {
        Some(c) => match c {
            'a' | 'o' | 'e' | 'i' | 'A' | 'O' | 'E' | 'I' => {
                piglatin_string.push(c);
                String::from("hay")
            }
            c => format!("{}ay", c),
        },
        _ => panic!("Empty string"),
    };

    println!("Current suffix: {}", suffix);
    loop {
        if let Some(c) = chars.next() {
            piglatin_string.push(c);
        } else {
            break;
        }
    }
    let piglatin_string = format!("{}-{}", piglatin_string, suffix);

    println!("Your word in piglatin: {}", piglatin_string);
}
