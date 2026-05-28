use crate::common::Problem;

mod matrix;

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(11, "Largest Product in a Grid")
        .with_answer(70600674)
        .solution("naive", naive::solve)
);
