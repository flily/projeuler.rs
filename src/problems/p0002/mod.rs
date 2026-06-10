use crate::common::Problem;

mod naive;
mod no_branch;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(2, "Even Fibonacci Numbers")
        .with_answer(4613732)
        .solution("naive", naive::solve)
        .solution("no_branch", no_branch::solve)
);
