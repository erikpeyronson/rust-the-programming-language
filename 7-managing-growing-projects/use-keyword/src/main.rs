
// Types are usually brought in to scope with full path apposed to functions where the parent module is brought.
use std::collections::HashMap;

// Exept when there are conflicting names
use std::fmt;
use std::io;

fn function1() -> fmt::Result {}
fn function2() -> io::Result {}

// the as keyword can be used for aliasing
use std::io::Result as IoResult;

// Use can also be nested with {}
use std::{cmp::Ordering, io};

// The glob operator brings in everything from a module
use std::collections::*;

fn function3() -> IoResult {}

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("HashMap: {:?}", map);
}
