#[test]

fn main() {
    let x = Box::new(5);

    let mut y = x;      // update this line, don't change other lines!

    *y = 4;

    assert_eq!(*y, 4);

    println!("Success!");
}