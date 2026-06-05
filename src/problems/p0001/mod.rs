use crate::common::Problem;

mod naive;
mod formula;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(1, "Multiples of 3 and 5")
        .with_answer(233168)
        .solution("naive", naive::solve)
        .solution("formula", formula::solve)
);
