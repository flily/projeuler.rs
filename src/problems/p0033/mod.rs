use crate::common::{Problem, SolutionInfo};

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 33,
    title: "Digit Cancelling Fractions",
    answer: 100,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "naive",
            entry: naive::solve,
        },
    ],
});
