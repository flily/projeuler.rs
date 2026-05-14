use crate::common::{Problem, SolutionInfo};

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 25,
    title: "1000-digit Fibonacci number",
    answer: 4782,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "naive BigInt",
            entry: naive::solve,
        },
        SolutionInfo {
            name: "naive BigInt (no array)",
            entry: naive::solve_no_array,
        },
    ],
});
