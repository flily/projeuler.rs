use crate::common::{Problem, SolutionInfo, load_data};

mod naive;
mod log;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 99,
    title: "Largest Exponential",
    answer: 709,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "naive",
            entry: naive::solve,
        },
        SolutionInfo {
            name: "logarithm",
            entry: log::solve,
        },
    ],
});

pub fn load() -> Vec<(i64, i64)> {
    let raw = load_data();
    let content = String::from_utf8(raw).unwrap();
    content.lines()
        .map(|line| {
            let nums = line.split(",")
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            (nums[0], nums[1])
        })
        .collect()
}
