use crate::common::Problem;

mod bruteforce;
mod flood;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(67, "Maximum Path Sum II")
        .with_answer(7273)
        .solution("bruteforce", bruteforce::solve)
        .solution("flood fill", flood::solve)
);

pub fn load() -> Vec<Vec<i64>> {
    let raw = Problem::load_data();
    let content = String::from_utf8(raw).unwrap();
    let mut result = Vec::new();
    
    content.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .for_each(|row| result.push(row));

    result
}
