use crate::common::{Problem, SolutionInfo, load_data};

mod bruteforce;
mod flood;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 67,
    title: "Maximum Path Sum II",
    answer: 7273,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "bruteforce",
            entry: bruteforce::solve,
        },
        SolutionInfo {
            name: "flood fill",
            entry: flood::solve,
        },
    ],
});

pub fn load() -> Vec<Vec<i64>> {
    let raw = load_data();
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
