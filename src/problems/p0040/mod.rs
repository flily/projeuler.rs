use crate::framework::Problem;

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(40, "Champernowne's Constant")
        .with_answer(210)
        .solution("naive", naive::solve)
);
