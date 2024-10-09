use std::io::{self, BufRead};

/*
 * Complete the 'countApplesAndOranges' function below.
 *
 * The function accepts the following parameters:
 *  1. INTEGER s - starting point of Sam's house.
 *  2. INTEGER t - ending point of Sam's house.
 *  3. INTEGER a - location of the apple tree.
 *  4. INTEGER b - location of the orange tree.
 *  5. INTEGER_ARRAY apples - distances at which each apple falls from the tree.
 *  6. INTEGER_ARRAY oranges - distances at which each orange falls from the tree.
 */

fn countApplesAndOranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    // Count apples that fall on the house
    let apple_count = apples.iter()
        .map(|&apple| a + apple) // Calculate where each apple falls
        .filter(|&position| position >= s && position <= t) // Check if it falls on the house
        .count(); // Count how many apples are in range

    // Count oranges that fall on the house
    let orange_count = oranges.iter()
        .map(|&orange| b + orange) // Calculate where each orange falls
        .filter(|&position| position >= s && position <= t) // Check if it falls on the house
        .count(); // Count how many oranges are in range

    // Print the results
    println!("{}", apple_count);
    println!("{}", orange_count);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let s = first_multiple_input[0];
    let t = first_multiple_input[1];

    let second_multiple_input: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let a = second_multiple_input[0];
    let b = second_multiple_input[1];

    let third_multiple_input: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let _m = third_multiple_input[0];
    let _n = third_multiple_input[1];

    let apples: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let oranges: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    countApplesAndOranges(s, t, a, b, &apples, &oranges);
}
