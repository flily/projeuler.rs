use super::load;

fn get_digits(num: i64) -> Vec<i64> {
    let mut digits = Vec::new();
    let mut n = num;
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }
    digits
}

fn update_relation(relations: &mut [(Vec<i64>, Vec<i64>)], a: i64, b: i64) {
    // b after a
    let (_, a_next) = &mut relations[a as usize];
    if !a_next.contains(&b) {
        a_next.push(b);
    }

    let (b_prev, _) = &mut relations[b as usize];
    if !b_prev.contains(&a) {
        b_prev.push(a);
    }
}

pub fn solve() -> i64 {
    let nums = load();
    let mut relations = vec![(Vec::new(), Vec::new()); 10];

    for num in nums {
        let digits = get_digits(num);
        for i in 0..(digits.len() - 1) {
            for j in i + 1..digits.len() {
                let a = digits[i];
                let b = digits[j];
                update_relation(&mut relations, a, b);
            }
        }
    }

    let mut digits = Vec::new();
    for (n, (prev, next)) in relations.iter().enumerate() {
        if prev.is_empty() && next.is_empty() {
            continue;
        }

        digits.push((n as i64, next.len() as i64));
    }
    digits.sort_by_key(|&(_, count)| count);

    let mut result = 0;
    for (digit, _) in digits {
        result = result * 10 + digit;
    }

    result
}
