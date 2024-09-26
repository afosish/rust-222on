#[test]

// Solve it in two ways
// DON'T let `println!` work

fn main() {
    use std::process::exit;
    never_return();

    println!("Failed!");
}

fn never_return()   {
    // Implement this function, don't modify the fn signatures
    panic!("panic!");
}