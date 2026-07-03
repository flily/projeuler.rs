use crate::framework::Problem;

mod num;

mod naive;
mod fp;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(8, "Largest Product in a Series")
        .with_answer(23514624000)
        .solution("naive", naive::solve)
        .solution("functional programming", fp::solve)
);
