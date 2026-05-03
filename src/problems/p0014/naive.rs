
fn collatz_seq_size(n: i64) -> i64 {
    let mut m = n;
    let mut result = 1i64;
    while m > 1 {
        m = if m % 2 == 0 {
            m / 2
        } else {
            3 * m + 1
        };
        result += 1;
    }

    return result;
}

pub fn solve_naive() -> i64 {
    let mut max_size = 0i64;
    let mut result = 0i64;

    for i in 1..1_000_000 {
        let size = collatz_seq_size(i);
        if size > max_size {
            max_size = size;
            result = i;
        }
    }

    return result;
}


#[cfg(test)]
mod tests {
    use crate::common::Checkable;
    
    use super::super::INFO;
    use super::collatz_seq_size;
    use super::solve_naive;

    #[test]
    fn test_collatz_seq_size() {
        assert_eq!(collatz_seq_size(13), 10);
    }

    #[test]
    fn test_solve_naive() {
        assert!(INFO.check(solve_naive()));
    }
}
