// FILL in the blanks.
trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}
#[test]
fn main() {
    let x = 1.1f64;
    let y = 8u8;

    // Draw x.
    draw_with_box(Box::new(x));

    // Draw y.
    draw_with_ref(&y);

    println!("Success!");
}

fn draw_with_box(x: Box<dyn Draw>) {
    println!("{}", x.draw()); // Call the draw method and print the result
}

fn draw_with_ref(x: &dyn Draw) {
    println!("{}", x.draw()); // Call the draw method and print the result
}
