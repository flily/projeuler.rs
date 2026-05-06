use crate::common::{Problem, SolutionInfo};

mod num;

mod naive;
mod fp;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 8,
    title: "Largest Product in a Series",
    answer: 23514624000,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "functional programming",
            entry: fp::solve,
        },
        SolutionInfo {
            name: "naive",
            entry: naive::solve,
        },
        SolutionInfo {
            name: "functional programming",
            entry: fp::solve,
        },
    ],
});
