#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn get_file(got_file: bool) -> Result<(), String> {
    if got_file {
        Ok(())
    } else {
        Err(String::from("didn't get a file"))
    }
}
pub fn add_two(x: i32) -> i32 {
    x + 2
}

pub fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct Guess {
    pub value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(101);

        let bar = "bar".repeat(2)[..];
        let my_string_literal = &"hello world"[0..1];
        let str = String::from("test")[..]
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 10,
            height: 5,
        };

        let smaller = Rectangle {
            width: 5,
            height: 2,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_not_hold_larger() {
        let larger = Rectangle {
            width: 10,
            height: 5,
        };

        let smaller = Rectangle {
            width: 5,
            height: 2,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    #[ignore]
    fn it_adds_two() {
        assert_eq!(add_two(10), 12);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        return get_file(true);
    }

    #[test]
    fn it_does_not_work() -> Result<(), String> {
        return get_file(false);
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}
