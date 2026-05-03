use crate::common::{Problem, SolutionInfo, load_data};

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 22,
    title: "Names scores",
    answer: 871198282,
    extra_time_ms: std::time::Duration::from_millis(0),
    solutions: vec![
        SolutionInfo {
            name: "naive",
            entry: naive::solve,
        },
    ],
});

pub fn load() -> Vec<String> {
    let content: Vec<u8> = load_data();

    content.split(|&c| c == b',')
        .map(|s| {
            // remove the leading and trailing double quotes
            s[1..s.len() - 1].iter()
            // convert to String
                .map(|&c| c as char)
                .collect()
        })
        .collect()
}
