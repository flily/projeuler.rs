use crate::common::{Problem, SolutionInfo};

mod naive;
mod sqrt;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 21,
    title: "Amicable Numbers",
    answer: 31626,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "naive",
            entry: naive::solve,
        },
        SolutionInfo {
            name: "sqrt",
            entry: sqrt::solve,
        },
    ],
});
