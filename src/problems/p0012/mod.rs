use crate::common::{Problem, SolutionInfo};

mod naive;
mod factor_filter;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 12,
    title: "Highly Divisible Triangular Number",
    answer: 76576500,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "naive",
            entry: naive::solve,
        },
        SolutionInfo {
            name: "factor filter (3 primes)",
            entry: factor_filter::solve_3,
        },
        SolutionInfo {
            name: "factor filter (6 primes)",
            entry: factor_filter::solve_6,
        },
        SolutionInfo {
            name: "factor filter (7 primes)",
            entry: factor_filter::solve_7,
        },
    ],
});
