use num_bigint::BigInt;

use super::load;

pub fn solve() -> i64 {
    let datas = load();
    let mut max_index = 0;
    let mut max_value = BigInt::from(0);

    for (index, (base, exp)) in datas.iter().enumerate() {
        let value = BigInt::from(*base).pow(*exp as u32);
        if value > max_value {
            max_value = value;
            max_index = index as i64;
        }
    }

    max_index + 1
}
