#[test]
fn chessboard()
{
    const SIZE: u8 = 8;

    for y in 0..SIZE {
        for x in 0..SIZE{
            let s = if (x+y) % 2 == 0 {"**"} else {"  "};
            print!("{s}")
        }
        println!();
    }
}
#[test]
fn chessboard1()
{
    const SIZE: u8 = 8;

    for y in 0..SIZE {
        for x in 0..SIZE{
            let s = match (x+y) % 2 {
                0 => "**",
                _ => "  ",
            };
            print!("{s}");
        }
        println!();
    }
}