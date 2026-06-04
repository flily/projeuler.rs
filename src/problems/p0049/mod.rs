use crate::common::Problem;

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(49, "Prime Permutations")
        .with_answer(296962999629)
        .solution("naive", naive::solve)
);
