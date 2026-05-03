
pub fn solve() -> i64 {
    let mut result = 0i64;

    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            result += i as i64;
        }
    }

    return result;
}
