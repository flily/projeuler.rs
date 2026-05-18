use crate::common::{Problem, SolutionInfo};

mod naive;
mod even_filter;
mod filter;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 35,
    title: "Circular Primes",
    answer: 55,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "naive",
            entry: naive::solve,
        },
        SolutionInfo {
            name: "filter even digits",
            entry: even_filter::solve,
        },
        SolutionInfo {
            name: "filter even digits and 5",
            entry: filter::solve,
        },
    ],
});
