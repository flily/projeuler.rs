use crate::common::{Problem, SolutionInfo};

mod naive;
mod generator;
mod euclid_formula;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 39,
    title: "Integer Right Triangles",
    answer: 840,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "naive",
            entry: naive::solve,
        },
        SolutionInfo {
            name: "order generator",
            entry: generator::solve,
        },
        SolutionInfo {
            name: "Euclid's formula",
            entry: euclid_formula::solve,
        },
    ],
});
