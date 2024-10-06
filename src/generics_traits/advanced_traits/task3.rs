trait Pilot {
    fn fly(&self) -> String;
}

trait Wizard {
    fn fly(&self) -> String;
}

struct Human;

impl Pilot for Human {
    fn fly(&self) -> String {
        String::from("This is your captain speaking.")
    }
}

impl Wizard for Human {
    fn fly(&self) -> String {
        String::from("Up!")
    }
}

impl Human {
    fn fly(&self) -> String {
        String::from("*waving arms furiously*")
    }
}
#[test]
fn main() {
    let person = Human;

    // Using fully qualified syntax to call the fly method from the Pilot trait
    assert_eq!(Pilot::fly(&person), "This is your captain speaking.");

    // Using fully qualified syntax to call the fly method from the Wizard trait
    assert_eq!(Wizard::fly(&person), "Up!");

    // Calling the inherent fly method on the Human struct
    assert_eq!(person.fly(), "*waving arms furiously*");

    println!("Success!");
}
