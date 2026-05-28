use crate::common::Problem;

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(29, "Distinct Powers")
        .with_answer(9183)
        .solution("naive", naive::solve)
);
