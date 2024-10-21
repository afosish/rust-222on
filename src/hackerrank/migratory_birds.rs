use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'migratoryBirds' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */
fn migratoryBirds(arr: &[i32]) -> i32 {
    // Create an array to store the frequency of bird sightings (for bird types 1 to 5).
    let mut frequency = vec![0; 5];

    // Count the occurrences of each bird type.
    for &bird in arr {
        if bird >= 1 && bird <= 5 {
            frequency[(bird - 1) as usize] += 1;
        }
    }

    // Find the bird type with the maximum count.
    let mut max_count = 0;
    let mut bird_id = 0;

    for (i, &count) in frequency.iter().enumerate() {
        if count > max_count {
            max_count = count;
            bird_id = i + 1; // bird types are 1-indexed
        }
    }

    bird_id as i32
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let _arr_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = migratoryBirds(&arr);

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();
    writeln!(&mut fptr, "{}", result).ok();
}
