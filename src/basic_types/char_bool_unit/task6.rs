#[test]

// Modify `4` in assert to make it work

fn main() {
    use std::mem::size_of_val;
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!");
}