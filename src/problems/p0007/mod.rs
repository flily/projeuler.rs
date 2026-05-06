use crate::common::{Problem, SolutionInfo};

mod naive;
mod sieve_full;
mod sieve_half;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 7,
    title: "10001st Prime",
    answer: 104743,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "naive",
            entry: naive::solve,
        },
        SolutionInfo {
            name: "sieve (full size)",
            entry: sieve_full::solve,
        },
        SolutionInfo {
            name: "sieve (half size)",
            entry: sieve_half::solve,
        },
    ],
});
