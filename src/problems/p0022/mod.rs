use crate::common::Problem;

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(22, "Names scores")
        .with_answer(871198282)
        .solution("naive", naive::solve)
);

pub fn load() -> Vec<String> {
    let content: Vec<u8> = Problem::load_data();

    content
        .split(|&c| c == b',')
        .map(|s| {
            // remove the leading and trailing double quotes
            s[1..s.len() - 1]
                .iter()
                // convert to String
                .map(|&c| c as char)
                .collect()
        })
        .collect()
}
