pub fn digit_square_sum(n: i64) -> i64 {
    let mut result = 0;
    let mut m = n;

    while m > 0 {
        let digit = m % 10;
        result += digit * digit;
        m /= 10;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::digit_square_sum;

    #[test]
    fn test_digit_square_sum() {
        assert_eq!(digit_square_sum(44), 32);
        assert_eq!(digit_square_sum(32), 13);
        assert_eq!(digit_square_sum(13), 10);
        assert_eq!(digit_square_sum(10), 1);

        assert_eq!(digit_square_sum(85), 89);
        assert_eq!(digit_square_sum(89), 145);
        assert_eq!(digit_square_sum(145), 42);
        assert_eq!(digit_square_sum(42), 20);
        assert_eq!(digit_square_sum(20), 4);
        assert_eq!(digit_square_sum(4), 16);
        assert_eq!(digit_square_sum(16), 37);
        assert_eq!(digit_square_sum(37), 58);
        assert_eq!(digit_square_sum(58), 89);
    }
}
