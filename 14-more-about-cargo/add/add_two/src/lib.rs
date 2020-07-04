/// Adds two to the number given.
///
/// # Examples
/// ```
/// let arg = 5;
/// let answer = add_two::add_two(arg);
///
/// assert_eq!(7, answer)
/// ```
pub fn add_two(x: i32) -> i32 {
    x + 2
}

#[cfg(tests)]
#[test]
fn it_works() {
    assert_eq!(4, add_two(2));
}
