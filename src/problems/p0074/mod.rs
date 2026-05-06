use crate::common::{Problem, SolutionInfo};

mod naive;
mod factorial_table;
mod cache;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 74,
    title: "Digit Factorial Chains",
    answer: 402,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "naive",
            entry: naive::solve,
        },
        SolutionInfo {
            name: "factorial table",
            entry: factorial_table::solve,
        },
        SolutionInfo {
            name: "cache",
            entry: cache::solve,
        },
    ],
});
