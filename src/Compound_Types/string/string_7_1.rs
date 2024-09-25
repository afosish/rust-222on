#[test]

// Fix error with at least two solutions
fn main() {
    let s = String :: from( "hello, world");
    greetings(s)
}

fn greetings(s: String) {
    println!("{}", s)
}