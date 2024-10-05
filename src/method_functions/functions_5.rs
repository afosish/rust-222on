struct Rectangle {
    width: u32,
    height: u32,
}

// First impl block for the area method
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// Second impl block for the can_hold method
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[test]
fn main() {
    println!("Success!");
}
