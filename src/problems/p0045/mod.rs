use crate::common::Problem;

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(45, "Triangular, Pentagonal, and Hexagonal")
        .with_answer(1533776805)
        .solution("naive", naive::solve)
);
