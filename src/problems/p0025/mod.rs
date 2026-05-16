use crate::common::{Problem, SolutionInfo};

mod naive;
mod tpmalachite;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 25,
    title: "1000-digit Fibonacci number",
    answer: 4782,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "naive BigInt (num-bigint)",
            entry: naive::solve,
        },
        SolutionInfo {
            name: "naive BigInt no array (num-bigint)",
            entry: naive::solve_no_array,
        },
        SolutionInfo {
            name: "BigInt (malachite)",
            entry: tpmalachite::solve,
        },
    ],
});
