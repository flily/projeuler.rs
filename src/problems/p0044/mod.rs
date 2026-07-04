use crate::framework::Problem;

mod naive;
mod search;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(44, "Pentagon Numbers")
        .with_answer(5482660)
        .solution("naive", naive::solve)
        .solution("search less, HashSet (rustc_hash)", search::solve_less_hashset)
        .solution("search larger, HashSet (rustc_hash)", search::solve_larger_hashset)
        .solution("search less, binary search", search::solve_less_bsearch)
        .solution("search larger, binary search", search::solve_larger_bsearch)
);
