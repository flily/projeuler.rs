use crate::framework::Problem;

mod naive;
mod array;
mod array_capacity;
mod array_prealloc;
mod sieve_full;
mod sieve_half;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| {
    Problem::init(7, "10001st Prime")
        .with_answer(104743)
        .solution("naive", naive::solve)
        .solution("array", array::solve)
        .solution("array (capacity)", array_capacity::solve)
        .solution("array (prealloc)", array_prealloc::solve)
        .solution("sieve (full size)", sieve_full::solve)
        .solution("sieve (half size)", sieve_half::solve)
});
