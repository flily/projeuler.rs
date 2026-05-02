use crate::common::Problem;
use crate::common::SolutionInfo;

use super::naive::solve_naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 1,
    title: "Multiples of 3 and 5".to_string(),
    answer: 233168,
    extra_time_ms: std::time::Duration::from_millis(0),
    solutions: vec![
        SolutionInfo {
            name: "Naive".to_string(),
            entry: solve_naive,
        },
    ],
});
