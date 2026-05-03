
pub fn solve_naive() -> i64 {
    let mut result = 0i64;

    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            result += i as i64;
        }
    }

    return result;
}


#[cfg(test)]
mod tests {
    use crate::common::Checkable;

    use super::super::INFO;
    use super::solve_naive;

    #[test]
    fn test_solve_naive() {
        assert!(INFO.check(solve_naive()));
    }
}
