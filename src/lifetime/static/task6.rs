use std::fmt::Display;
#[test]
fn main() {
    let mut string = "First".to_owned();

    string.push_str(string.to_uppercase().as_str());
    print_a(&string);
    print_b(&string);
    print_c(&string); // Compilation error
    print_d(&string); // Compilation error
    print_e(&string);
    print_f(&string);
    print_g(&string); // Compilation error
}

fn print_a<T: Display + 'static>(t: &T) {
    println!("{}", t);
}

fn print_b<T>(t: &T)
where
    T: Display + 'static,
{
    println!("{}", t);
}

fn print_c(t: &dyn Display) {
    println!("{}", t)
}

fn print_d(t: &impl Display) {
    println!("{}", t)
}

fn print_e(t: &(dyn Display + 'static)) {
    println!("{}", t)
}

fn print_f(t: &(impl Display + 'static)) {
    println!("{}", t)
}

fn print_g(t: &String) {
    println!("{}", t);
}