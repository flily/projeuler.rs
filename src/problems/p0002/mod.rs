use crate::common::{Problem, SolutionInfo};

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 2,
    title: "Even Fibonacci Numbers",
    answer: 4613732,
    extra_time_ms: std::time::Duration::from_millis(0),
    solutions: vec![
        SolutionInfo {
            name: "naive",
            entry: naive::solve,
        },
    ],
});
