use crate::framework::Problem;

mod tpnumbigint;
mod tpmalachite;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(53, "Combinatoric Selections")
        .with_answer(4075)
        .solution("naive use BigInt (num-bigint)", tpnumbigint::solve)
        .solution("factorial table BigInt (num-bigint)", tpnumbigint::solve_precalculated)
        .solution("naive use BigInt (malachite)", tpmalachite::solve)
        .solution("factorial table BigInt (malachite)", tpmalachite::solve_precalculated)
);
