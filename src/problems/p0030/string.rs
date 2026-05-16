fn digit_sum_of_powers(n: i64, pow: u32) -> i64 {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|d| i64::pow(d as i64, pow))
        .sum()
}

pub fn solve() -> i64 {
    let mut count = 0;
    let max =  i64::pow(9, 5) * 6;

    for n in 2..max {
        if n == digit_sum_of_powers(n, 5) {
            count += n;
        }
    }

    count
}
