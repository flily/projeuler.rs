use crate::framework::Problem;

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(65, "Convergents of e")
        .with_answer(272)
        .solution("naive", naive::solve)
);
