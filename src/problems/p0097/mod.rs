use crate::common::Problem;

mod modulo;
mod tpnumbigint;
mod tpmalachite;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(97, "Large Non-Mersenne Prime")
        .with_answer(8739992577)
        .solution("multiply with modulo", modulo::solve)
        .solution("naive BigInt (num-bigint)", tpnumbigint::solve)
        .solution("naive BigInt (malachite)", tpmalachite::solve)
);
