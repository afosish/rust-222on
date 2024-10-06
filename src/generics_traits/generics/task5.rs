
// Add generic for Val to make the code work, DON'T modify the code in `main`.
struct Val <L> {
    val: L,
}

impl <L> Val <L> {
    fn value(&self) -> &L {
        &self.val
    }
}

#[test]
fn main() {
    let x = Val{ val: 3.0 };
    let y = Val{ val: "hello".to_string()};
    println!("{}, {}", x.value(), y.value());
}