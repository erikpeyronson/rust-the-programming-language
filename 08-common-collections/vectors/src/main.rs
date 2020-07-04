fn main() {
    // Empty vector needs type
    let v: Vec<i32> = Vec::new();
    println!("Empty vector {:?}", v);

    // but initialized with the vec macro element type can be infered
    let mut v = vec![1, 2, 3];
    println!("Vector with values{:?}", v);

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("Same vector with som more values added with push {:?}", v);

    let third: &i32 = &v[2];
    println!("The third element is with index is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element with get is {}", third),
        None => println!("There is no third element"),
    }

    let v = vec![1, 2, 3, 4, 5];
    // Indexing a none existant value with bracets panics
    // let does_not_exist = &v[100];
    // While get returns "None"
    let does_not_exist = v.get(100);
    println!("does_not_exist: {:?}", does_not_exist);

    /*
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];
    // Cant push here since first is borrowed as unmutable and since pushing might reorganize the memory the reference could be invalid
    v.push(6);

    println!("The first element is {}", first);
    */

    // Iterating over values in a vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // Iterating and modifying
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }

    // Enums can be used to store diffent types in a vector
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Blue")),
        SpreadsheetCell::Float(10.22),
    ];
}
