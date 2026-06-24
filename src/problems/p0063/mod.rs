use crate::common::Problem;

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(63, "Powerful Digit Counts")
        .with_answer(49)
        .solution("naive", naive::solve)
);
