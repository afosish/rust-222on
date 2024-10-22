/* Make it work in two ways */
use std::fmt::Display;

fn foobar_1(thing: &dyn Display) {}

fn foobar_2(thing: Box<dyn Display>) {}
#[test]
fn main() {
}