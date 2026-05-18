use crate::common::{Problem, SolutionInfo};

mod naive;
mod facttable;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 34,
    title: "Digit Factorials",
    answer: 40730,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "naive",
            entry: naive::solve,
        },
        SolutionInfo {
            name: "factorial table",
            entry: facttable::solve,
        },
    ],
});
