fn main() {
    // Create string with new()
    let s = String::new();
    println!("Empty string s: {}", s);

    // Create string with iniial data
    let data = "Initial contents";
    let s = data.to_string();
    println!("String with data: {}", s);

    // also work on a litteral
    let s = "Initial data".to_string();
    println!("String created from literal: {}", s);

    // String::from() is equivalent
    let s = String::from("Initial data");
    println!("String with string::from() {},", s);

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("Pushed string: {}", s);

    // Push string does not take ownership of the parameter
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("S1: {}, s2 {}", s1, s2);

    // Push pushes a single character
    let mut s = String::from("lo");
    s.push('l');
    println!("s: {}", s);

    // Concatenation using the + operator
    let s1 = String::from("Hello");
    let s2 = String::from(", World");
    let s3 = s1 + &s2; // s1 is moved and can no longer be used
    println!("S1: {}, s3: {}", s2, s3);

    // For more compilcated concatenations format macro is prefered
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // "Зд" is 4 bytes
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);

    // Iterating over string using the chars method
    for c in "Здравствуйте".chars() {
        println!("{}", c);
    }
    // Iterating over string using the bytes method
    for c in "Здравствуйте".bytes() {
        println!("{}", c);
    }
}
