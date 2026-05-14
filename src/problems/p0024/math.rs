static FACTORIALS: [i64; 10] = [
    1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880,
];


fn to_i64(digits: &[i64]) -> i64 {
    let mut result = 0;
    for &d in digits {
        result = result * 10 + d;
    }
    result
}

const LIMIT: i64 = 1_000_000;

pub fn solve() -> i64 {
    let mut digits_left = 9;
    let mut numbers_left = LIMIT - 1;
    let mut digits_use = [false; 10];
    let mut digits_pick = Vec::new();

    while digits_left >= 0 {
        let block_count = FACTORIALS[digits_left as usize];
        let number_index = numbers_left / block_count;
        
        let (mut i, mut j) = (0, 0);
        while i < digits_use.len() {
            if !digits_use[i] {
                if j == number_index {
                    digits_use[i] = true;
                    digits_pick.push(i as i64);
                    break;
                }
                j += 1;
            }
            i += 1;
        }

        numbers_left -= number_index * block_count;
        digits_left -= 1;
    }

    to_i64(&digits_pick)
}

