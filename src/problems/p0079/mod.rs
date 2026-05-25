use crate::common::{Problem, SolutionInfo, load_data};

mod naive;
mod sort;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 79,
    title: "Passcode Derivation",
    answer: 73162890,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "naive",
            entry: naive::solve,
        },
        SolutionInfo {
            name: "sort",
            entry: sort::solve,
        },
    ],
});

pub fn load() -> Vec<i64> {
    let raw = load_data();
    let content = String::from_utf8(raw).unwrap();
    content.lines().map(|line| line.parse::<i64>().unwrap()).collect()
}
