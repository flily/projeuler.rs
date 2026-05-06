use crate::common::{Problem, SolutionInfo};

mod naive;
mod cache_hashmap;
mod cache_vector;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 14,
    title: "Longest Collatz sequence",
    answer: 837799,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "naive",
            entry: naive::solve,
        },
        SolutionInfo {
            name: "with cache (HashMap)",
            entry: cache_hashmap::solve,
        },
        SolutionInfo {
            name: "with cache (Vector)",
            entry: cache_vector::solve,
        },
    ],
});
