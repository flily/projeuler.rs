use crate::common::{Problem, SolutionInfo};

mod nums;

mod tpbigintlib;
mod stringint;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 13,
    title: "Large Sum",
    answer: 5537376230,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "BitInt (num-bigint)",
            entry: tpbigintlib::solve,
        },
        SolutionInfo {
            name: "bigint by Vec<u8>",
            entry: stringint::solve,
        },
    ],
});
