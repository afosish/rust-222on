use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::collections::HashMap;

/*
 * Complete the 'sockMerchant' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts the following parameters:
 *  1. INTEGER n
 *  2. INTEGER_ARRAY ar
 */

fn sockMerchant(_n: i32, ar: &[i32]) -> i32 {
    // Create a HashMap to count the occurrences of each sock color
    let mut color_count = HashMap::new();

    // Count occurrences of each sock color
    for &sock in ar {
        let count = color_count.entry(sock).or_insert(0);
        *count += 1;
    }

    // Calculate the number of pairs
    let mut pairs = 0;
    for &count in color_count.values() {
        pairs += count / 2;  // Each pair is formed by 2 socks
    }

    pairs
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = sockMerchant(n, &ar);

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();
    writeln!(&mut fptr, "{}", result).ok();
}
