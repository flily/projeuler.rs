use super::load;

fn triangle_number(n: i64) -> i64 {
    n * (n + 1) / 2
}

fn word_score(word: &str) -> i64 {
    let mut result = 0;
    for c in word.chars() {
        result += (c as u8 - b'A' + 1) as i64;
    }

    result
}

pub fn solve() -> i64 {
    let triangle_numbers = (1..20).map(triangle_number).collect::<Vec<i64>>();

    let mut result = 0;
    for word in load() {
        let score = word_score(&word);
        if triangle_numbers.contains(&score) {
            result += 1;
        }
    }

    result
}
