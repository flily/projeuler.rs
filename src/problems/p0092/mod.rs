use crate::common::{Problem, SolutionInfo};

mod common;

mod naive;
mod cache_hashset;
mod cache_vector;
mod cache_vectorlet;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 92,
    title: "Square digit chains",
    answer: 8581146,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "naive",
            entry: naive::solve,
        },
        SolutionInfo {
            name: "with cache (HashSet)",
            entry: cache_hashset::solve,
        },
        SolutionInfo {
            name: "with cache (Vector)",
            entry: cache_vector::solve,
        },
        SolutionInfo {
            name: "with cache (reduced Vector)",
            entry: cache_vectorlet::solve,
        },
    ],
});
