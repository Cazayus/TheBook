fn main() {}

#[derive(Debug)]
struct Rectangle {
    _width: u32,
    _height: u32,
}

impl Rectangle {
    fn _can_hold(&self, other: &Rectangle) -> bool {
        self._width > other._width && self._height > other._height
    }
}
fn _add_two(a: i32) -> i32 {
    a + 2
}
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, _add_two(2));
        assert_ne!(5, _add_two(2));
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    #[should_panic(expected = "Make this test fail")]
    fn another() {
        panic!("Make this test fail");
    }
    #[test]
    fn it_works2() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            _width: 8,
            _height: 7,
        };
        let smaller = Rectangle {
            _width: 5,
            _height: 1,
        };

        assert!(larger._can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            _width: 8,
            _height: 7,
        };
        let smaller = Rectangle {
            _width: 5,
            _height: 1,
        };

        assert!(!smaller._can_hold(&larger));
    }
}
