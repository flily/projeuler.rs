use crate::common::{Problem, SolutionInfo};

mod naive;
mod string;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 30,
    title: "Digit Fifth Powers",
    answer: 443839,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "naive",
            entry: naive::solve,
        },
        SolutionInfo {
            name: "string",
            entry: string::solve,
        },
    ],
});
