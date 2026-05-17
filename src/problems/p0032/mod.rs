use crate::common::{Problem, SolutionInfo};

mod naive;
mod filter;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 32,
    title: "Pandigital Products",
    answer: 45228,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "naive",
            entry: naive::solve,
        },
        SolutionInfo {
            name: "filter",
            entry: filter::solve,
        },
    ],
});
