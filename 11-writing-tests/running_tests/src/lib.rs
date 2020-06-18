#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[derive(Debug)]
    pub struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.height * self.width
        }
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.height > other.height && self.width > other.width
        }
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }

    struct Guess {
        value: i32,
    }

    impl Guess {
        fn new(value: i32) -> Guess {
            if value < 1 {
                panic!(
                    "Guess value must be greater than or equal to 1, got {}.",
                    value
                );
            } else if value > 100 {
                panic!(
                    "Guess value must be less than or equal to 100, got {}.",
                    value
                );
            }
            Guess { value }
        }
        fn value(&self) -> i32 {
            self.value
        }
    }

    pub fn add_two(a: i32) -> i32 {
        // a + 3
        a + 2
    }

    pub fn greeting(name: &str) -> String {
        format!("Hello {}!", name)
        // format!("Hello")
    }

    pub fn prints_and_returns_10(a: i32) -> i32 {
        println!("I got the value {}", a);
        10
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn exploration() {
            assert_eq!(2 + 2, 4);
        }
        #[test]
        // fn another() {
        //     panic!("Make this test fail");
        // }
        #[test]
        fn larger_can_hold_smaller() {
            let larger = Rectangle {
                width: 8,
                height: 7,
            };
            let smaller = Rectangle {
                width: 5,
                height: 1,
            };

            assert!(larger.can_hold(&smaller));
        }
        #[test]
        fn smaller_cannot_hold_larger() {
            let larger = Rectangle {
                width: 8,
                height: 7,
            };
            let smaller = Rectangle {
                width: 5,
                height: 1,
            };
            assert!(!smaller.can_hold(&larger));
        }

        #[test]
        fn it_adds_two() {
            assert_eq!(4, add_two(2), "Failed to add two");
        }

        #[test]
        fn greeting_contains_name() {
            let result = greeting("carol");
            assert!(
                result.contains("carol"),
                "Greeting did not contain name, value was {}",
                result
            );
        }

        #[test]
        #[should_panic(expected = "Guess value must be less than or equal to 100")]
        fn greater_than_100() {
            Guess::new(200);
        }
        #[test]
        fn it_works() -> Result<(), String> {
            if 2 + 2 == 4 {
                Ok(())
            } else {
                Err(String::from("Two plus two does not equal four"))
            }
        }
        #[test]
        fn this_test_will_pass() {
            let value = prints_and_returns_10(4);
            assert_eq!(10, value);
        }
        #[test]
        #[ignore]
        fn this_test_will_fail() {
            let value = prints_and_returns_10(4);
            assert_eq!(5, value);
        }
    }
}
