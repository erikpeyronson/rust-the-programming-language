use std::collections::HashMap;

fn main() {
    // Constructing map using insert
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("red"), 20);

    println!("Scores {:?}", scores);

    // Accessing values in a hash map
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("team_name: {}, score: {:?}", team_name, score);

    // Iterate over map
    for (key, value) in &scores {
        println!("{}, {}", key, value);
    }

    // Overwriting a value
    println!("Before update {:?}", scores);
    scores.insert(String::from("Blue"), 100);
    println!("After update {:?}", scores);

    // Only inserting if the key has no value
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(500);
    println!(
        "After using entry to add 'Yellow' and not update 'Blue': {:?}",
        scores
    );

    // Updating a Value based on the Old value
    let text = "Hello world oh wonderful world world full of wonders";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("Occurences: {:?}", map);

    // Constructing map using iterators and collect
    let teams = vec![String::from("blue"), String::from("Red")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("Scores: {:?}", scores);

    // Things that implement the copy trait are copied into the map
    let mut map = HashMap::new();
    let num1 = 1;
    let num2 = 2;
    map.insert(num1, num2);
    println!("Map: {:?}, num1: {}, num2: {}", map, num1, num2);

    // not possible with for example string
    let mut map = HashMap::new();
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    map.insert(field_name, field_value);
    // Compile error: field_name and field_value is moved into
    // println!("Map: {:?}, field_name: {}, filed_value: {}", map, field_name, field_value);
}
