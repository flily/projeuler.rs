use crate::common::{Problem, SolutionInfo};

mod naive;
mod remove_factor;
mod prime_table;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 3,
    title: "Largest Prime Factor",
    answer: 6857,
    extra_time_ms: std::time::Duration::from_millis(0),
    solutions: vec![
        SolutionInfo {
            name: "naive",
            entry: naive::solve,
        },
        SolutionInfo {
            name: "remove factors",
            entry: remove_factor::solve,
        },
        SolutionInfo {
            name: "prime table",
            entry: prime_table::solve,
        },
    ],
});
