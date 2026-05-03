fn collatz_seq_size(n: i64) -> i64 {
    let mut m = n;
    let mut result = 1i64;
    while m > 1 {
        m = if m % 2 == 0 { m / 2 } else { 3 * m + 1 };
        result += 1;
    }

    result
}

pub fn solve() -> i64 {
    let mut max_size = 0i64;
    let mut result = 0i64;

    for i in 1..1_000_000 {
        let size = collatz_seq_size(i);
        if size > max_size {
            max_size = size;
            result = i;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::collatz_seq_size;

    #[test]
    fn test_collatz_seq_size() {
        assert_eq!(collatz_seq_size(13), 10);
    }
}
