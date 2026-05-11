use crate::common::{Problem, SolutionInfo};

mod tpbigintlib;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 20,
    title: "Factorial Digit Sum",
    answer: 648,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "BigInt (num-bigint) to_string()",
            entry: tpbigintlib::solve_string,
        },
        SolutionInfo {
            name: "BigInt (num-bigint) by math",
            entry: tpbigintlib::solve_math,
        },
    ],
});
