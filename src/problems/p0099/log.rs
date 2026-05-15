use super::load;

pub fn solve() -> i64 {
    let datas = load();
    let mut max_index = 0;
    let mut max_log_value = 0.0;

    // when b1 ^ e1 > b2 ^ e2
    // => ln(b1 ^ e1) > ln(b2 ^ e2)
    // => e1 * ln(b1) > e2 * ln(b2)
    for (index, (base, exp)) in datas.iter().enumerate() {
        let b = *base as f64;
        let e = *exp as f64;
        let log_value = e * b.ln();
        if log_value > max_log_value {
            max_log_value = log_value;
            max_index = index as i64;
        }
    }

    max_index + 1
}
