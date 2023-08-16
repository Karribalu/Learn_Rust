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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_hold_test() {
        let a = Rectangle {
            width: 10,
            height: 30,
        };
        let b = Rectangle {
            width: 5,
            height: 40,
        };
        assert!(a.can_hold(&b));
    }
}
