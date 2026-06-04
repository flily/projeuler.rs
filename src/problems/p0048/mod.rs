use crate::common::Problem;

mod powermod;
mod tpnumbigint;
mod tpmalachite;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(48, "Self Powers")
        .with_answer(9110846700)
        .solution("power with modulo", powermod::solve)
        .solution("BigInt (num-bigint)", tpnumbigint::solve)
        .solution("BigInt (malachite-base)", tpmalachite::solve)
);
