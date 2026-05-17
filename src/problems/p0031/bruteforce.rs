fn counts_iter(expected: i64, total: i64, index: usize, coin_max: &[i64], coin_types: &[i64]) -> i64 {
    let mut result = 0;
    if index >= coin_max.len() {
        result = if total == expected { 1 } else { 0 };
    } else {
        for i in 0..=coin_max[index] {
            let change = i * coin_types[index];
            result += counts_iter(expected, total + change, index + 1, coin_max, coin_types);
        }
    }

    result
}

fn count_coins(pences: i64, coins_type: &[i64]) -> i64 {
    let mut coin_max = vec![0; coins_type.len()];
    for (i, c) in coins_type.iter().enumerate() {
        coin_max[i] = ((pences as u64) / (*c as u64)) as i64;
    }

    counts_iter(pences, 0, 0, &coin_max, coins_type)
}

const COINS: [i64; 8] = [200, 100, 50, 20, 10, 5, 2, 1];

pub fn solve() -> i64 {
    count_coins(200, &COINS)
}
