pub fn solve() -> i64 {
    let mut square_sum = 0;
    for i in 1..101 {
        square_sum += i * i;
    }

    let mut sum_square = 0;
    for i in 1..101 {
        sum_square += i;
    }

    sum_square * sum_square - square_sum
}