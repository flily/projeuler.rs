use crate::framework::Problem;

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(19, "Counting Sundays")
        .with_answer(171)
        .solution("naive", naive::solve)
);
