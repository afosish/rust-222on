use rand::Rng;
fn count_permutation(shipments: &Vec<u32>) -> Option<usize> {
    let n = shipments.len() as u32;
    let total_weight: u32 = shipments.iter().sum();

    if total_weight % n != 0 {
        return None;
    }

    let target_weight = total_weight / n;
    let mut moves = 0;
    let mut cumulative_imbalance = 0;

    for &weight in shipments {
        let imbalance = weight as i32 - target_weight as i32;
        cumulative_imbalance += imbalance;
        moves += cumulative_imbalance.abs() as usize;
    }

    Some(moves)
}

fn gen_shipments(n: usize) -> Vec<u32> {
    let target_weight = rand::thread_rng().gen_range(1..=100);
    vec![target_weight; n]
}
#[test]
fn main() {
    let shipments = gen_shipments(5);
    println!("Ваги кораблів: {:?}", shipments);
    match count_permutation(&shipments) {
        Some(moves) => println!("Мінімальна кількість перенесень: {}", moves),
        None => println!("Неможливо рівномірно розподілити вантаж."),
    }
}
