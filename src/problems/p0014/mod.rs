use crate::common::{Problem, SolutionInfo};

mod naive;
mod cache;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 14,
    title: "Longest Collatz sequence",
    answer: 837799,
    extra_time_ms: std::time::Duration::from_millis(0),
    solutions: vec![
        SolutionInfo {
            name: "naive",
            entry: naive::solve_naive,
        },
        SolutionInfo {
            name: "with cache",
            entry: cache::solve_cache,
        },
    ],
});
