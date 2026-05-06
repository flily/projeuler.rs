use super::num::NUMBER;

pub fn solve() -> i64 {
    let num: Vec<i64> = NUMBER.chars().map(|c| (c as i64) - 0x30).collect();
    let mut max_production = 0i64;

    for i in 0..(num.len() - 13) {
        let production = num[i..(i + 13)].iter().product();
        if production > max_production {
            max_production = production;
        }
    }

    max_production
}
