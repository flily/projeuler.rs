use crate::common::Problem;

mod naive;
mod permutation;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(41, "Pandigital Prime")
        .with_answer(7652413)
        .solution("naive", naive::solve)
        .solution("permutation", permutation::solve)
);
