use super::load;

fn name_score(name: &str) -> i64 {
    let mut result = 0i64;

    for c in name.chars() {
        result += (c as u8 - b'A' + 1) as i64;
    }

    return result;
}

pub fn solve() -> i64 {
    let mut result = 0i64;

    let mut names = load();
    names.sort();

    for (i, name) in names.iter().enumerate() {
        let index = (i + 1) as i64;
        let score = name_score(name);
        result += index * score;
    }

    return result;
}
