use crate::common::Problem;

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(38, "Pandigital Multiples")
        .with_answer(932718654)
        .solution("naive", naive::solve)
);
