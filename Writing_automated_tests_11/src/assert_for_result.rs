
#[derive(Debug)]
pub struct Rectangle {
    pub length: u32,
    pub width: u32
}

impl Rectangle {
    pub fn can_hold (&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

#[cfg(test)]
pub mod tests {
    use super::*; // this ensures that anything defined in the outer module is available in this inner module, in case you decide to use this module in a different rust file
    #[test]
    pub fn another_name() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    pub fn larger_can_hold_smaller() {
        let larger = Rectangle {length: 20, width: 50};
        let smaller = Rectangle {length:3, width:30};

        assert!(larger.can_hold(&smaller)); //the correct (expected) result is used in the assert! macro
    }
}