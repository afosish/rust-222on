use std::fmt;

// Define the newtype `Pretty` as a tuple struct that wraps a String.
struct Pretty(String);

impl fmt::Display for Pretty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use self.0 to access the wrapped String inside Pretty
        write!(f, "\"{}\"", self.0.clone() + ", world")
    }
}
#[test]
fn main() {
    let w = Pretty("hello".to_string());
    println!("w = {}", w); // This will print: w = "hello, world"
}
