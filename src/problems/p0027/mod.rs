use crate::common::{Problem, SolutionInfo};

mod naive;
mod cache;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 27,
    title: "Quadratic Primes",
    answer: -59231,
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
    ],
});
