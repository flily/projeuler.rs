use crate::framework::Problem;

mod set;
mod set_early_return;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(61, "Cyclical Figurate Numbers")
        .with_answer(28684)
        .solution("with set", set::solve)
        .solution("with set (early return)", set_early_return::solve)
);
