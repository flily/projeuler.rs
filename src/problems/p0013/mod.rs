use crate::common::Problem;

mod nums;

mod tpnumbigint;
mod stringint;
mod tpmalachite;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(13, "Large Sum")
        .with_answer(5537376230)
        .solution("BigInt (num-bigint)", tpnumbigint::solve)
        .solution("bigint by Vec<u8>", stringint::solve)
        .solution("BigInt (malachite)", tpmalachite::solve)
);
