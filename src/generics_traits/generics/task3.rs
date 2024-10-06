struct Point<T> {
    x: T,
    y: T,
}
// Implement struct Point to make it work.

#[test]
fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("Success!");
}