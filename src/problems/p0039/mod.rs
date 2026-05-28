use crate::common::Problem;

mod naive;
mod generator;
mod euclid_formula;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(39, "Integer Right Triangles")
        .with_answer(840)
        .solution("naive", naive::solve)
        .solution("order generator", generator::solve)
        .solution("Euclid's formula", euclid_formula::solve)
);
