use itertools::Itertools;

fn main() {
    let solutions = find_solutions();
    for (m, u, x, a, s, l, o, n) in &solutions {
        println!("{}{}{}{} x {} = {}{}{}{}", m, u, x, a, a, s, l, o, n);
    }
    println!("Загальна кількість рішень: {}", solutions.len());
}

fn find_solutions() -> Vec<(u8, u8, u8, u8, u8, u8, u8, u8)> {
    let mut results = Vec::new();

    // Перебір всіх перестановок чисел від 1 до 8
    for perm in (1..=8).permutations(8) {
        let m = perm[0];
        let u = perm[1];
        let x = perm[2];
        let a = perm[3];
        let s = perm[4];
        let l = perm[5];
        let o = perm[6];
        let n = perm[7];

        // Обчислюємо числа "Муха" і "Слон"
        let muxa = 1000 * m + 100 * u + 10 * x + a;
        let slon = 1000 * s + 100 * l + 10 * o + n;

        // Перевірка умови задачі
        if muxa * a == slon {
            results.push((m, u, x, a, s, l, o, n));
        }
    }

    results
}
