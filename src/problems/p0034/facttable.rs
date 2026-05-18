
const FACT_TABLE: [i64; 10] = [
    1, // 0!
    1, // 1!
    2, // 2!
    6, // 3!
    24, // 4!
    120, // 5!
    720, // 6!
    5040, // 7!
    40320, // 8!
    362880, // 9!
];

fn is_sum_of_digital_factorials(n: i64) -> bool {
    let mut sum = 0;
    let mut m = n;

    while m > 0 {
        sum += FACT_TABLE[(m % 10) as usize];
        m /= 10;
    }

    sum == n
}

pub fn solve() -> i64 {
    let mut sum = 0;
    let max_n = FACT_TABLE[9] + FACT_TABLE[8];
    for n in 3..=max_n {
        if is_sum_of_digital_factorials(n) {
            sum += n;
        }
    }

    sum
}
