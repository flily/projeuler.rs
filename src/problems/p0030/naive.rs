fn digit_sum_of_powers(n: i64, pow: u32) -> i64 {
    let mut sum = 0;
    let mut m = n;
    while m > 0 {
        let d = m % 10;
        sum += i64::pow(d, pow);
        m /= 10;
    }

    sum
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
