use crate::framework::Problem;

mod naive;
mod string;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(30, "Digit Fifth Powers")
        .with_answer(443839)
        .solution("naive", naive::solve)
        .solution("string", string::solve)
);
