use crate::common::Problem;

mod naive;
mod logfactor;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(5, "Smallest Multiple")
        .with_answer(232792560)
        .solution("naive", naive::solve)
        .solution("find factor by log", logfactor::solve)
);
