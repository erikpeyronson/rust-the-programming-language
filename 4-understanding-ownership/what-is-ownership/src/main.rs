fn main() {
    // Stack allocated string literal
    let s = "hello!";
    println!("{}", s);

    // Heap allocated string type
    {
        let mut s = String::from("Hello");
        println!("{}", s);
        s.push_str(", world!");
        println!("{}", s)
    } // drop is called and string memory is released

    let s1 = String::from("hello");
    println!("{}, world!", s1);
    let s2 = s1;
    // Not possible since s2 owns the string
    // println!("{}, world!", s1);
    println!("{}, world!", s2);

    // To do a deep copy clone is used
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1: {}, s2: {}", s1, s2);

    // Stack allocated types copes instead due to having the Copy trait.
    let x = 5;
    let y = x;
    println!("X:{}, Y:{}", x, y);

    let s1 = String::from("hello");
    takes_ownership(s1);
    // Not possible
    // println!("{}", s1);

    let i1 = 6;
    makes_copy(i1);
    println!("{}", i1);

    let s1 = String::from("hello");
    let s2 = takes_and_gives_back(s1);
    // Possible
    println!("{}", s2);

    let s1 = String::from("A long string with several characters");
    let (s2, len) = calculate_length(s1);
    println!("'{}' has length:{}", s2, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
    // some_string goes out of scope and is dropped.
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
    // copied and not dropped
}

fn takes_and_gives_back(some_string: String) -> String {
    println!("{}", some_string);
    some_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
