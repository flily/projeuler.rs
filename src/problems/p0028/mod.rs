use crate::common::{Problem, SolutionInfo};

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 28,
    title: "Number Spiral Diagonals",
    answer: 669171001,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "naive counting",
            entry: naive::solve,
        },
    ],
});
