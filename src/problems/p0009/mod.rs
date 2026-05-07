use crate::common::{Problem, SolutionInfo};

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 9,
    title: "Special Pythagorean Triplet",
    answer: 31875000,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "naive",
            entry: naive::solve,
        },
    ],
});
