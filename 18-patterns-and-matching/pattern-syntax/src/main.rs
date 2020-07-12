struct Point {
    x: i32,
    y: i32,
}

struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum ImprovedMessage {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum HelloMessage {
    Hello { id: i32 },
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the Y parameter: {}", y);
}

fn main() {
    // Matching literals
    let x = 1;

    match x {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Anything"),
    }

    // Match shadows y in the following example
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    // Multiple patterns
    let x = 1;

    match x {
        1 | 2 => println!("One or Two"),
        3 => println!("Three"),
        _ => println!("Anything"),
    }

    // Matching ranges
    let x = 5;

    match x {
        1..=5 => println!("One through five"),
        _ => println!("Something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("Early ASCCI letter"),
        'k'..='z' => println!("Late ASCCI letter"),
        _ => println!("Something else"),
    }

    // Destructuring structs
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // Shorter syntax if variables are named as the struct fields
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // Matching some literals and deconstruct other
    match p {
        Point { x, y: 0 } => println!("On the x asis at {}", x),
        Point { x: 0, y } => println!("On the y asis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // Destructuring enums
    // let msg = Message::ChangeColor(0, 160, 255);
    // let msg = Message::Quit;
    // let msg = Message::Move { x: 2, y: 3 };
    let msg = Message::Write(String::from("Pattern matching i awesome"));

    match msg {
        Message::Quit => println!("The quit variant has no data to destructure"),
        Message::Move { x, y } => {
            println!("Move in the x direction {}, and in the y direction{}", x, y)
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }

    // Destructuring nested structs and enums
    let msg = ImprovedMessage::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        ImprovedMessage::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        ImprovedMessage::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, sturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }

    // Destructing structs and tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("Feet {} inches {}, ({}, {})", feet, inches, x, y);

    // Ignoring values in a pattern
    foo(3, 4);

    // Ignoring parts of a value with a nested _
    // let mut settings_value = None;
    let mut settings_value = Some(5);
    let new_settings_value = Some(10);

    match (settings_value, new_settings_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            settings_value = new_settings_value;
        }
    }

    println!("Setting is: {:?}", settings_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }
    // ignoring unused variables
    let _x = 5;
    let y = 10;

    let s = Some(String::from("hello"));

    // The commented one will not work since _s bind the value and takes ownership of the string while _ does not
    // if let Some(_s) = s {
    if let Some(_) = s {
        println!("found a string")
    }
    println!("{:?}", s);

    // .. can be used to ignore the remaining parts ofa value

    let origin = Point3d { x: 0, y: 0, z: 0 };

    match origin {
        Point3d { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    // Extra conditionals with match guards
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("Less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case x = {:?}", x),
    }

    println!("At the end: x = {:?}, y = {}", x, y);

    let x = 4;
    let y = false;
    // let y = true;

    match x {
        4 | 5 | 6 if y => println!("Yes"),
        _ => println!("No"),
    }

    let msg = HelloMessage::Hello { id: 5 };

    match msg {
        HelloMessage::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        HelloMessage::Hello { id: 10..=12 } => println!("Found id in another range"),
        HelloMessage::Hello { id } => println!("Found another id: {}", id),
    }
}
