#[test]

// Make it work, don't modify `implicitly_ret_unit` !
fn main() {
    let _v: () = ();

    let v = (2, 3);
    assert_eq!(v, implicitly_ret_unit());

    println!("Success!");
}

fn implicitly_ret_unit() -> (i32,i32)
{
    println!("I will return a ()");
    (2,3)
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}