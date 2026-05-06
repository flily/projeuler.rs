use crate::common::{Problem, SolutionInfo};

mod naive;
mod logfactor;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 5,
    title: "Smallest Multiple",
    answer: 232792560,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "naive",
            entry: naive::solve,
        },
        SolutionInfo {
            name: "find factor by log",
            entry: logfactor::solve,
        },
    ],
});
