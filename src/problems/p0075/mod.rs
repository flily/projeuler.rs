use crate::framework::Problem;

mod naive;
mod search;
mod formula;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(75, "Singular Integer Right Triangles")
        .with_answer(161667)
        .solution("naive", naive::solve)
        .solution("search int a and b, then check c", search::solve)
        .solution("Pell's Equation formula", formula::solve)
);
