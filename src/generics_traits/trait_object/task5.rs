trait MyTrait {
    type Output;
    fn f(&self) -> Self::Output;
}

impl MyTrait for u32 {
    type Output = u32;
    fn f(&self) -> Self::Output { 42 }
}

impl MyTrait for String {
    type Output = String;
    fn f(&self) -> Self::Output { self.clone() }
}

fn my_function<T: MyTrait>(x: T)  {
    x.f();
}
#[test]
fn main() {
    my_function(13_u32);
    my_function(String::from("abc"));

    println!("Success!");
}
