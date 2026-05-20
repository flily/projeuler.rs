pub fn solve() -> i64 {
    let primes : [i64; 8] = [2, 3, 5, 7, 11, 13, 17, 19];
    let mut result = 1;

    for p in primes {
        let x = 20.0f64.log(p as f64).floor() as i64;
        result *= p.pow(x as u32);
    }

    result
}
