// Struct with named fields
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = build_user(
        String::from("erik.peyrnson@gmail.com"),
        String::from("Erik Peyronson"),
    );
    user1.email = String::from("erik.peyronson@gmail.com");
    // println!("user1: {}", user1);

    // Struct update syntax
    let user2 = User{
        email: String::from("another.user@domain.com"),
        username: String::from("Another User"),
        ..user1
        // This:
        // is Equivalent with:
        // acvie: user1.active,
        // sign_in_count: user1.sign_in_count,
    };
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        // The above is identical to below:
        // email: email,
        // username: username,
        active: true,
        sign_in_count: 1,
    }
}
