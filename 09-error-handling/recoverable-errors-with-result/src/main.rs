use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

// Main can return error which allows error propagation with ?
fn main() -> Result<(), Box<dyn Error>> {
    let f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(e) => panic!("Could not open file {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file {:?}", other_error);
            }
        },
    };
    // Equivalent with the above but using a closure to clean up
    // nested match
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating file {:?}");
            })
        } else {
            panic!("Problem opening the file");
        }
    });

    // The result types have methods to avoid having to match
    // for example unwrap who calls panic
    let f = File::open("hello.txt").unwrap();

    // Expect works like unwrap but allows for entering a message
    let f = File::open("hello.txt").expect("Failed to open file");

    let s = read_username_from_file().unwrap();
    let s = read_username_from_file_improved();
    let s = read_username_from_file_more_improved();
    let s = read_username_from_file_best()?;
    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// The ? operator can be used to propagate errors up the stack
// as long ass the error can be converted to the resulting type.
fn read_username_from_file_improved() -> Result<String, io::Error> {
    let mut f = File::open("Hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// The ? return value can be chained
fn read_username_from_file_more_improved() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// But the best way would be to use the file into string directly
fn read_username_from_file_best() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
