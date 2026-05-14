// This is a template solution file. You can modify it as needed.

fn to_i64(digits: &[i64]) -> i64 {
    let mut result = 0;
    for &d in digits {
        result = result * 10 + d;
    }
    result
}

fn generate_permutations(digits: &mut Vec<bool>, got: &mut Vec<i64>, i: i64, count: i64, stop_at: i64) -> (i64, i64) {
    if i >= digits.len() as i64 {
        let c = count + 1;
        let v = if c >= stop_at {
            to_i64(got)
        } else {
            0
        };

        return (c, v);
    }

    let mut j = 0;
    let mut found_count = count;
    while j < digits.len() {
        if !digits[j] {
            digits[j] = true;
            got[i as usize] = j as i64;
            let (c, value) = generate_permutations(digits, got, i + 1, found_count, stop_at);
            got[i as usize] = -1;
            digits[j] = false;
            found_count = c;
            if value > 0 {
                return (c, value);
            }
        }
        j += 1;
    }

    (found_count, 0)
}

const LIMIT: i64 = 1_000_000;

pub fn solve() -> i64 {
    let mut got = vec![0; 10];
    let mut digits = vec![false; 10];
    let (_, result) = generate_permutations(&mut digits, &mut got, 0, 0, LIMIT);
    result
}
