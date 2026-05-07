use crate::common::{Problem, SolutionInfo};

mod naive;
mod sieve;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 10,
    title: "Summation of Primes",
    answer: 142913828922,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "naive",
            entry: naive::solve,
        },
        SolutionInfo {
            name: "sieve",
            entry: sieve::solve,
        },
    ],
});
