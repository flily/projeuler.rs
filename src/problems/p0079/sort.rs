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
    // b > a
    let (_, a_greater) = &mut relations[a as usize];
    if !a_greater.contains(&b) {
        a_greater.push(b);
    }

    let (b_less, _) = &mut relations[b as usize];
    if !b_less.contains(&a) {
        b_less.push(a);
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
    for (i, (less, greater)) in relations.iter().enumerate() {
        if less.is_empty() && greater.is_empty() {
            continue;
        }

        digits.push((i as i64, greater.len() as i64));
    }
    digits.sort_by_key(|&(_, count)| count);
    let mut result = 0;
    for (digit, _) in digits {
        result = result * 10 + digit;
    }

    result
}
