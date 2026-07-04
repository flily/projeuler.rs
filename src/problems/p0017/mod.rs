use crate::framework::Problem;

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(17, "Number Letter Counts")
        .with_answer(21124)
        .solution("naive", naive::solve)
);
