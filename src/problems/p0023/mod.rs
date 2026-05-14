use crate::common::{Problem, SolutionInfo};

mod naive;
mod cache;
mod substraction;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 23,
    title: "Non-Abundant Sums",
    answer: 4179871,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "naive",
            entry: naive::solve,
        },
        SolutionInfo {
            name: "cache",
            entry: cache::solve,
        },
        SolutionInfo {
            name: "substraction",
            entry: substraction::solve,
        },
    ],
});
