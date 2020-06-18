use adder;

mod common;
// Integration test only uses the public api
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
