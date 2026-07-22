use crate::framework::Problem;

mod bruteforce_malachite;
mod bruteforce_mod;
mod partition;
mod partition_mod;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(78, "Coin Partitions")
        .with_answer(55374)
        .solution("bruteforce (malachite)", bruteforce_malachite::solve)
        .solution("bruteforce (mod)", bruteforce_mod::solve)
        .solution("partition function (cached)", partition::solve)
        .solution("partition function (cached, mod)", partition_mod::solve)
);
