fn main() {
    // Mixing if let else if let else
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // While let
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];

    // Matching in for
    for (index, value) in v.iter().enumerate() {
        println!("{}, is at index {}", value, index);
    }

    // Matching with let
    let (x, y, z) = (1, 2, 3);
    println!("({}, {}, {})", x, y, z);

    // Matching in function header and deconstructing a tuple
    let point = (3, 5);
    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location, ({}, {})", x, y);
}
