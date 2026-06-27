use crate::common::Problem;

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(68, "Magic 5-gon Ring")
        .with_answer(6531031914842725)
        .solution("naive", naive::solve)
);
