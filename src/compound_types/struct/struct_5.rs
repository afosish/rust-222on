
// Fill the blank
struct Person {
    name: String,
    age: u8,
}
#[test]
fn main() {
    println!("Success!");
}

fn build_person(name: String, age: u8) -> Person {
    Person {
        name,
        age,
        
    }
}