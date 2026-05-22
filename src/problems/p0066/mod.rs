use crate::common::{Problem, SolutionInfo};

mod naive;
mod formula;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 66,
    title: "Diophantine Equation",
    answer: 661,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "naive",
            entry: naive::solve,
        },
        SolutionInfo {
            name: "Pell's Equation formula",
            entry: formula::solve,
        },
    ],
});
