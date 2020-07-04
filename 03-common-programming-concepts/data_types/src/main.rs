fn main() {
    // numeric operations
    let x = 2.0;
    let y: f32 = 3.0;
    println!("X: {}, Y: {}", x, y);

    let sum = 5 + 10;
    println!("5 + 10 = {}", sum);

    // Char
    let heart_eyed_cat = '😻';
    println!("Cat: {:?}", heart_eyed_cat);

    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("This is a tuple {:?}", tup);

    let (x, y, z) = tup;
    println!("The tuple split up x:{}, y:{}, z:{}", x, y, z);

    println!("The tuple indexed {} {} {}", tup.0, tup.1, tup.2);

    // array
    let a = [1, 2, 3, 4, 5, 6];
    println!("Array: {:?}, array len: {}", a, a.len());

    let a: [i32; 5];
    a = [1, 2, 3, 4, 5];
    println!("Array: {:?}, array len: {}", a, a.len());

    let a = [3; 5];
    println!("Array: {:?}, array len: {}", a, a.len());

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("First element: {}, second element: {}", first, second);

    // Compiles with crash
    let index = 10;
    let element = a[index];
    println!("Illegal index {}", element).
}
