use crate::common::{Problem, SolutionInfo};

mod matrix;

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 11,
    title: "Largest Product in a Grid",
    answer: 70600674,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "naive",
            entry: naive::solve,
        },
    ],
});
