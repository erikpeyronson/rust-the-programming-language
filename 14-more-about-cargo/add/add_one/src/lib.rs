//! `add_one` is a crate containing a function to add one to a number

/// Adds one to the number given.
///
/// # Examples
/// ```
/// let arg = 5;
/// let answer = add_one::add_one(arg);
///
/// assert_eq!(6, answer)
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(tests)]
#[test]
fn it_works() {
    assert_eq!(3, add_one(2));
}
