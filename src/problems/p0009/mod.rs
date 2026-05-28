use crate::common::Problem;

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
     Problem::init(9, "Special Pythagorean Triplet")
        .with_answer(31875000)
        .solution("naive", naive::solve)
);
