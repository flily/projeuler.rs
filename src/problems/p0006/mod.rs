use crate::common::Problem;

mod naive;
mod fp;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(6, "Sum Square Difference")
        .with_answer(25164150)
        .solution("loop", naive::solve)
        .solution("functional programming", fp::solve)
);
