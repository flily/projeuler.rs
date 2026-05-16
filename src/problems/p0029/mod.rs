use crate::common::{Problem, SolutionInfo};

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 29,
    title: "Distinct Powers",
    answer: 9183,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "naive",
            entry: naive::solve,
        },
    ],
});
