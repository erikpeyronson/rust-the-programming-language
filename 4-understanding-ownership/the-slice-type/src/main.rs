fn main() {
    let s = String::from("Hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    let s = String::from("Hello world");
    let hello = &s[..5];
    let world = &s[6..];
    println!("&s[..5]:{}, &s[6..]{}", hello, world);

    let s = String::from("Hello");
    let len = s.len();
    let slice = &s[3..len];
    println!("&s[3..len]: {}", slice);

    let slice = &s[3..];
    println!("&s[3..]: {}", slice);

    let slice = &s[..];
    println!("&s[..]: {}", slice);

    let s = String::from("Hello World");
    let first = first_word(&s);
    println!("first_word(\"Hello World\"): {}", first);

    // Compile error to use the slice after clearing s.
    // let mut s = String::from("Hello World");
    // let first = first_word(&s);
    // s.clear();
    // println!("first_word() after s.clear()", {});

    let my_string_literal = "Hello World";
    let word = first_word(&my_string_literal);
    println!("first_world(string_litteral: {}", word);

    let numbers = [1, 2, 3, 4, 5, 6];
    let num_slice = &numbers[3..];
    println!("Slice of numbers {:?}", num_slice);
}

// Returning index of first space, bad since index can be stored until after the string has been cleared
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// Returning slice
// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     &s[..]
// }

// Both taking and returning slice better
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
