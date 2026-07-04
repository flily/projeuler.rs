use crate::framework::Problem;

mod naive;
mod even_filter;
mod filter;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(35, "Circular Primes")
        .with_answer(55)
        .solution("naive", naive::solve)
        .solution("filter even digits", even_filter::solve)
        .solution("filter even digits and 5", filter::solve)
);
