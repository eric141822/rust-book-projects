pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
pub struct Rectangle {
    width: usize,
    height: usize,
}

#[derive(Debug)]
pub struct Guess {
    value: usize,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Guess {
    pub fn new(value: usize) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value)
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.", value)
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        if add(2, 2) == 4 {
            Ok(())
        } else {
            Err(String::from("Two plus two does not equal four"))
        }
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 10,
            height: 10,
        };
        let smaller = Rectangle {
            width: 5,
            height: 5,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 10,
            height: 10,
        };
        let smaller = Rectangle {
            width: 5,
            height: 5,
        };

        assert!(!smaller.can_hold(&larger));
    }
}
