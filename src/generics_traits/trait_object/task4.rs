trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Foo for String {
    fn method(&self) -> String {
        format!("string: {}", *self)
    }
}

// Implement static dispatch using generics
fn static_dispatch<T: Foo>(x: T) {
    println!("{}", x.method());
}

// Implement dynamic dispatch using trait objects
fn dynamic_dispatch(x: &dyn Foo) {
    println!("{}", x.method());
}
#[test]
fn main() {
    let x = 5u8;
    let y = "Hello".to_string();

    static_dispatch(x);        // Uses static dispatch
    dynamic_dispatch(&y);      // Uses dynamic dispatch

    println!("Success!");
}
