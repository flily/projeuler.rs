use crate::common::Problem;

mod naive;
mod statemachine;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(59, "XOR Decryption")
        .with_answer(129448)
        .solution("naive", naive::solve)
        .solution("state machine", statemachine::solve)
);

pub fn load() -> Vec<i64> {
    let content = Problem::load_data();
    
    content.split(|&c| c == b',')
        .map(|s| {
            let s = std::str::from_utf8(s).unwrap();
            s.parse().unwrap()
        })
        .collect()
}