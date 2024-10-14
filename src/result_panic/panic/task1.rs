
// FILL the blanks
fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success!");
        // IMPLEMENT the below code
        panic!("End!")

    }

    println!("Exercise Failed if printing out this line!");
}
#[test]
fn main() {
    drink("lemonade");

    println!("Exercise Failed if printing out this line!");
}