use crate::framework::Problem;

mod naive;
mod formula;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(85, "Counting Rectangles")
        .with_answer(2772)
        .solution("naive", naive::solve)
        .solution("formula", formula::solve)
);
