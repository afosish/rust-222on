use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn timeConversion(s: &str) -> String {
    // Extract the hour, minute, second, and period (AM/PM)
    let hour_str = &s[0..2];
    let minute = &s[3..5];
    let second = &s[6..8];
    let period = &s[8..10]; // AM or PM

    let mut hour: i32 = hour_str.parse().unwrap();

    // Convert based on AM/PM
    if period == "AM" {
        if hour == 12 {
            hour = 0; // 12 AM is midnight, so we convert it to 00
        }
    } else if period == "PM" {
        if hour != 12 {
            hour += 12; // For PM, add 12 to the hour unless it's 12 PM
        }
    }

    // Format the time in 24-hour format
    format!("{:02}:{:02}:{:02}", hour, minute, second)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    // Read input string
    let s = stdin_iterator.next().unwrap().unwrap();

    // Get the result from the function
    let result = timeConversion(&s);

    // Write the result to the output file
    writeln!(&mut fptr, "{}", result).ok();
}
