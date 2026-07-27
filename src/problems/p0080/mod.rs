use crate::framework::Problem;

mod viete_tpnumbigint;
mod viete_tpmalachite;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(80, "Square Root Digital Expansion")
        .with_answer(40886)
        .solution("viete's method (num-bigint)", viete_tpnumbigint::solve)
        .solution("viete's method (malachite)", viete_tpmalachite::solve)
);
