fn to_vec_int(s: &str) -> Vec<u8> {
    s.chars().rev().map(|c| c as u8 - b'0').collect()
}

fn vec_int_add(a: &[u8], b: &[u8]) -> Vec<u8> {
    let max_size = a.len().max(b.len());
    let mut result = Vec::with_capacity(max_size + 1);
    let mut carry = 0;

    for i in 0..max_size {
        let ai = if i < a.len() { a[i] } else { 0 };
        let bi = if i < b.len() { b[i] } else { 0 };

        let sum = ai + bi + carry;
        result.push(sum % 10);
        carry = if sum >= 10 { 1 } else { 0 };
    }

    if carry > 0 {
        result.push(carry);
    }

    result
}

pub fn solve() -> i64 {
    let mut sum = to_vec_int("0");
    for n in super::nums::NUMS {
        let num_vec = to_vec_int(n);
        sum = vec_int_add(&sum, &num_vec);
    }

    let mut result = 0;
    let base = sum.len() - 10;
    let ten = 10i64;
    for (i, _) in sum.iter().enumerate().skip(base) {
        let c = sum[i] as i64;
        result += c * ten.pow((i - base) as u32);
    }

    result
}
