fn main() {
    let s = String::from("Hello");

    let len = calculate_length(&s);

    println!("The length of '{}' is: {}", s, len);

    let mut s = String::from("Hello");
    println!("{}", s);
    change(&mut s);
    println!("after change(s): {}", s);

    // A value can only have one mutable reference
    // let r1 = &mut s;
    // Compile error
    // let r2 = &mut s;
    // println!("s:{}, r1:{}, r2:{}", s, r1, r2);

    // Can be combined as long the unmutable references are not used after the mutable reference has been instaniated
    let mut s = String::from("Hello");

    let r1 = &s;
    let r2 = &s;
    println!("r1:{}, r2:{}", r1, r2);

    let r3 = &mut s;
    println!("r3:{}", r3);

    println!("no_dangle(): {}", no_dangle());
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scoped since the reference does not own the value

// Non mutable references cannot be changed;
// fn change(some_string: &String) {
//     some_string.push_str(", world!");
// }

fn change(some_string: &mut String) {
    some_string.push_str(", World");
}

// Not possible
// fn dangle(some_string) -> {
//     let s = String::from("Hello");

//     &s
// }

fn no_dangle() -> String {
    let s = String::from("Hello");
    s // Ownership is moved out
}
