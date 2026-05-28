use crate::common::Problem;

mod naive;
mod sqrt;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(21, "Amicable Numbers")
        .with_answer(31626)
        .solution("naive", naive::solve)
        .solution("sqrt", sqrt::solve)
);
