use crate::framework::Problem;

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(73, "Counting Fractions in a Range")
        .with_answer(7295372)
        .solution("naive", naive::solve)
);
