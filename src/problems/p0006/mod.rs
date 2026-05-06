use crate::common::{Problem, SolutionInfo};

mod naive;
mod fp;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 6,
    title: "Sum Square Difference",
    answer: 25164150,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "loop",
            entry: naive::solve,
        },
        SolutionInfo {
            name: "functional programming",
            entry: fp::solve,
        },
    ],
});
