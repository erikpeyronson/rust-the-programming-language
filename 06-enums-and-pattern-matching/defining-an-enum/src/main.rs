#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    // Using struct with enum field
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("192.168.1.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("Home: {:#?}", home);
    println!("Loopback: {:#?}", loopback);

    // Using only enum
    let home = IpAddress::V4(192, 168, 2, 2);
    let loopback = IpAddress::V6(String::from("::1"));
    println!("Home: {:#?}", home);
    println!("Loopback: {:#?}", loopback);

    // The option enum
    let some_number = Some(5);
    let some_string = Some("A string");
    let absend_number: Option<i32> = None;

    println!(
        "some_number: {:?}, some_string:{:?}, absent_number: {:?}",
        some_number, some_string, absend_number
    );
}
