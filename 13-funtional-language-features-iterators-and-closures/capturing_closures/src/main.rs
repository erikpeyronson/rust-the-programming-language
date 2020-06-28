use capturing_closures::Counter;

fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;
    assert!(equal_to_x(y), "Y is not equal to x");

    let x = vec![1, 2, 3];

    let equal_to_x = |z| z == x;

    println!("Can use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));

    let equal_to_x = move |z| z == x;

    // println!("Cant use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));

    let counter = Counter::new();
    for i in counter {
        println!("Item {}", i);
    }
}
