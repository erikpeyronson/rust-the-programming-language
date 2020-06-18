// Placing this file in a common directory makes the compiler treat it as a non test
// so that can be called from multiple tests

pub fn setup() {
    println!("Common setup function that can be used by multiple integration tests");
}
