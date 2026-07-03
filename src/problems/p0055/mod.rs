use crate::framework::Problem;

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(55, "Lychrel Numbers")
        .with_answer(249)
        .solution("naive", naive::solve)
);
