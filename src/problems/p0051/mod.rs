use crate::common::Problem;

mod naive;
mod math;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(51, "Prime Digit Replacements")
        .with_answer(121313)
        .solution("naive", naive::solve)
        .solution("math", math::solve)
);
