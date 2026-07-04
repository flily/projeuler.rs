use crate::framework::Problem;

mod naive;
mod filter;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(32, "Pandigital Products")
        .with_answer(45228)
        .solution("naive", naive::solve)
        .solution("filter", filter::solve)
);
