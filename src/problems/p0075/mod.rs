use crate::common::{Problem, SolutionInfo};

mod naive;
mod search;
mod formula;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 75,
    title: "Singular Integer Right Triangles",
    answer: 161667,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "naive",
            entry: naive::solve,
        },
        SolutionInfo {
            name: "search int a and b, then check c",
            entry: search::solve,
        },
        SolutionInfo {
            name: "Pell's Equation formula",
            entry: formula::solve,
        },
    ],
});
