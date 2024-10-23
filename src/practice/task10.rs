use std::time::{SystemTime, UNIX_EPOCH};
struct SimpleRng {
    seed: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        SimpleRng { seed }
    }

    fn next(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(1664525).wrapping_add(1013904223);
        (self.seed >> 16) as u32
    }

    fn gen_range(&mut self, low: i32, high: i32) -> i32 {
        (self.next() % ((high - low) as u32)) as i32 + low
    }
}

fn gen_random_vector(n: usize) -> Vec<i32> {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let mut rng = SimpleRng::new(seed);
    (0..n).map(|_| rng.gen_range(10, 100)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (usize, usize, i32) {
    data.windows(2)
        .enumerate()
        .map(|(i, pair)| (i, pair[0] + pair[1]))
        .min_by_key(|&(_, sum)| sum)
        .map(|(i, sum)| (i, i + 1, sum))
        .unwrap()
}

fn display_vector(data: &[i32]) {
    let n = data.len();

    println!("indexes: {}", (0..n).map(|i| format!("{:>3}.", i)).collect::<Vec<_>>().join(" "));

    println!("data:    {}", data.iter().map(|&v| format!("{:>3}", v)).collect::<Vec<_>>().join(", "));

    let (i1, i2, min_sum) = min_adjacent_sum(data);
    println!("indexes: {}", (0..n).map(|i| {
        if i == i1 {
            "\\__".to_string()
        } else if i == i2 {
            "__/".to_string()
        } else {
            "    ".to_string()
        }
    }).collect::<Vec<_>>().join(" "));

    println!("min adjacent sum = {} + {} = {} at indexes: {}, {}", data[i1], data[i2], min_sum, i1, i2);
}
#[test]
fn main() {
    let data = gen_random_vector(20);

    display_vector(&data);
}
