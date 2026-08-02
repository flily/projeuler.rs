use crate::framework::Problem;

mod naive;
mod formula;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(94, "Almost Equilateral Triangles")
        .with_answer(518408346)
        .solution("naive", naive::solve)
        .solution("formula", formula::solve)
);
