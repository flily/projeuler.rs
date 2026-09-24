use crate::framework::Problem;

mod naive;
mod naiveodds;
mod logfactor;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(5, "Smallest Multiple")
        .with_answer(232792560)
        .solution("naive", naive::solve)
        .solution("naive odds", naiveodds::solve)
        .solution("find factor by log", logfactor::solve)
);
