pub fn solve() -> i64 {
    let square_sum = (1..101).map(|x| x * x).sum::<i64>();
    let sum_square = (1..101).sum::<i64>();

    sum_square * sum_square - square_sum
}
