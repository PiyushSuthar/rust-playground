#[derive(Debug)]
pub struct Rect {
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }
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
    fn larger_can_hold_smaller() {
        let larger = Rect {
            width: 10,
            height: 9,
        };

        let smaller = Rect {
            width: 4,
            height: 5,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
