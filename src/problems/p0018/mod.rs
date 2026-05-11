use crate::common::{Problem, SolutionInfo};

mod triangle;

mod bruteforce;
mod flood;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 18,
    title: "Maximum Path Sum I",
    answer: 1074,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "bruteforce (recursive)",
            entry: bruteforce::solve,
        },
        SolutionInfo {
            name: "flood fill",
            entry: flood::solve,
        },
    ],
});
