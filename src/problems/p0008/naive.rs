use super::num::NUMBER;

pub fn solve() -> i64 {
    let num: Vec<char> = NUMBER.chars().collect();
    let mut max_production = 0;

    for i in 0..(num.len() - 13) {
        let mut production = 1;
        for j in 0..13 {
            production *= (num[i + j] as i64) - 0x30;
        }

        if production > max_production {
            max_production = production;
        }
    }

    max_production
}
