use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'countingValleys' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER steps
 *  2. STRING path
 */

fn countingValleys(steps: i32, path: &str) -> i32 {
    let mut prev_level = 0;
    let mut count = 0;
    for c in path.chars() {
        let curr_level = match c {
            'U' => prev_level + 1,
            'D' => prev_level - 1,
            _ => prev_level,
        };
        if curr_level == 0 && prev_level < 0 {
            count += 1;
        }
        prev_level = curr_level;
    }
    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let steps = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let path = stdin_iterator.next().unwrap().unwrap();

    let result = countingValleys(steps, &path);

    writeln!(&mut fptr, "{}", result).ok();
}
