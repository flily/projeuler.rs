use crate::common::Problem;

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(28, "Number Spiral Diagonals")
        .with_answer(669171001)
        .solution("naive counting", naive::solve)
);
