use crate::common::{Problem, SolutionInfo};

mod naive;
mod generator;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 36,
    title: "Double-base Palindromes",
    answer: 872187,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "naive",
            entry: naive::solve,
        },
        SolutionInfo {
            name: "10-palindromes generator",
            entry: generator::solve,
        },
    ],
});
