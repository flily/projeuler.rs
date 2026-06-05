fn sum_of_multiples(n: i64, k: i64) -> i64 {
    let m = (n - 1) / k;
    k * m * (m + 1) / 2
}


pub fn solve() -> i64 {
    let n = 1000;
    sum_of_multiples(n, 3) + sum_of_multiples(n, 5) - sum_of_multiples(n, 15)
}
