use crate::common::{Problem, SolutionInfo};

mod naive;
mod math;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 24,
    title: "Lexicographic Permutations",
    answer: 2783915460,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "naive",
            entry: naive::solve,
        },
        SolutionInfo {
            name: "math",
            entry: math::solve,
        },
    ],
});
