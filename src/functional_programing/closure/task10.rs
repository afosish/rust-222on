/* Fill in the blank and fix the error */
fn create_fn() -> Box<dyn Fn(i32) -> i32> {
    let num = 5;

    // how does the following closure capture the environment variable `num`
    // &T, &mut T, T ?
    Box::new(move |x| x + num)
}

#[test]
fn main() {
    let fn_plain = create_fn();
    fn_plain(1);
}