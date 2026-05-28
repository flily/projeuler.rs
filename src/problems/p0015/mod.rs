use crate::common::Problem;

mod calc;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(15, "Lattice Paths")
        .with_answer(137846528820)
        .solution("naive", calc::solve)
);
