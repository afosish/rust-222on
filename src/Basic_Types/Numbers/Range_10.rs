#[test]
// Fill the blanks

fn main() {
    use std::ops::{Range, RangeInclusive};
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}