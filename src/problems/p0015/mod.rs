use crate::common::{Problem, SolutionInfo};

mod calc;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 15,
    title: "Lattice Paths",
    answer: 137846528820,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "naive",
            entry: calc::solve,
        },
    ],
});
