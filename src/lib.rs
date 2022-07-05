#[cfg(test)]
mod tests {
    // use outer module
    use super::*;
    // autoinsert this code
    // use crate::Rectangle;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic]
    fn another() {
        // BACKTRACE puts panic trace, but test passed
        panic!("panic at disco");
    }

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
    fn it_adds_two() {
        // Rust test is no order expected, actual
        assert_eq!(4, add_two(2));
        assert_ne!(3, add_two(2));
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
