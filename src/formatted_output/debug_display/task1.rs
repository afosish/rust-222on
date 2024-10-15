#[derive(Debug)]
/* Fill in the blanks and Fix the errors */
struct Structure(i32);
#[test]
fn main() {
    // Types in std and Rust have implemented the fmt::Debug trait
    println!("{:?} months in a year.", 12);

    println!("Now {:?} will print!", Structure(3));
}