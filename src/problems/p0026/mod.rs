use crate::common::{Problem, SolutionInfo};

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 26,
    title: "Reciprocal Cycles",
    answer: 983,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "naive",
            entry: naive::solve,
        },
        SolutionInfo {
            name: "on prime denominators",
            entry: naive::solve_prime,
        },
    ],
});
