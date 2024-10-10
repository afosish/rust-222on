#[test]
fn main() {
    // impl From<bool> for i32
    let i1: i32 = false.into();
    let i2: i32 = i32::from(false);
    assert_eq!(i1, i2);
    assert_eq!(i1, 0);

    // FIX the error in two ways
    /* 1. use a similar type which `impl From<char>`, maybe you
    should check the docs mentioned above to find the answer */
    // 2. a keyword from the last chapter
    let i3: i32 = 'a' as i32;

    // FIX the error in two ways
    let s: String = 'a'.to_string();

    println!("Success!");
}