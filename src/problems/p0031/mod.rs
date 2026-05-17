use crate::common::{Problem, SolutionInfo};

mod bruteforce;
mod branchcut;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 31,
    title: "Coin Sums",
    answer: 73682,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "naive",
            entry: bruteforce::solve,
        },
        SolutionInfo {
            name: "branch cut",
            entry: branchcut::solve,
        },
    ],
});
