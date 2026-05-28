use crate::common::Problem;

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(33, "Digit Cancelling Fractions")
        .with_answer(100)
        .solution("naive", naive::solve)
);
